use chrono::{DateTime, UTC};

use opcua_types::*;

use opcua_core::comms::secure_channel::SecureChannel;

use address_space::address_space::AddressSpace;
use subscriptions::subscriptions::Subscriptions;
use server::ServerState;
use continuation_point::BrowseContinuationPoint;

/// Session info holds information about a session created by CreateSession service
#[derive(Clone)]
pub struct SessionInfo {}

/// Structure that captures diagnostics information for the session
#[derive(Clone)]
pub struct SessionDiagnostics {}

impl SessionDiagnostics {
    pub fn new() -> SessionDiagnostics {
        SessionDiagnostics {}
    }
}

const MAX_DEFAULT_PUBLISH_REQUEST_QUEUE_SIZE: usize = 100;
const PUBLISH_REQUEST_TIMEOUT: i64 = 30000;


/// The Session is any state maintained between the client and server
pub struct Session {
    /// Subscriptions associated with the session
    pub subscriptions: Subscriptions,
    /// The session identifier
    pub session_id: NodeId,
    /// Flag to indicate session should be terminated
    pub terminate_session: bool,
    /// Security policy
    pub security_policy_uri: String,
    /// Client's certificate
    pub client_certificate: ByteString,
    /// Authentication token for the session
    pub authentication_token: NodeId,
    /// Secure channel state
    pub secure_channel: SecureChannel,
    /// Session nonce
    pub session_nonce: ByteString,
    /// Session timeout
    pub session_timeout: Double,
    /// User identity token
    pub user_identity: Option<ExtensionObject>,
    /// Negotiated max request message size
    pub max_request_message_size: UInt32,
    /// Negotiated max response message size
    pub max_response_message_size: UInt32,
    /// Endpoint url for this session
    pub endpoint_url: UAString,
    /// Maximum number of continuation points
    max_browse_continuation_points: usize,
    /// Browse continuation points (oldest to newest)
    browse_continuation_points: Vec<BrowseContinuationPoint>,
    /// Diagnostics associated with the session
    diagnostics: SessionDiagnostics,
    /// Indicates if the session has received an ActivateSession
    pub activated: bool,
    /// Time that session was terminated, helps with recovering sessions, or clearing them out
    terminated_at: DateTime<UTC>,
    /// Flag indicating session is actually terminated
    pub terminated: bool,
    /// Internal value used to create new session ids.
    last_session_id: UInt32,
}

impl Session {
    #[cfg(test)]
    pub fn new_no_certificate_store() -> Session {
        let max_publish_requests = MAX_DEFAULT_PUBLISH_REQUEST_QUEUE_SIZE;
        let max_browse_continuation_points = super::constants::MAX_BROWSE_CONTINUATION_POINTS;
        Session {
            subscriptions: Subscriptions::new(max_publish_requests, PUBLISH_REQUEST_TIMEOUT),
            session_id: NodeId::null(),
            activated: false,
            terminate_session: false,
            terminated: false,
            terminated_at: UTC::now(),
            client_certificate: ByteString::null(),
            security_policy_uri: String::new(),
            authentication_token: NodeId::null(),
            secure_channel: SecureChannel::new_no_certificate_store(),
            session_nonce: ByteString::null(),
            session_timeout: 0f64,
            user_identity: None,
            max_request_message_size: 0,
            max_response_message_size: 0,
            endpoint_url: UAString::null(),
            max_browse_continuation_points,
            browse_continuation_points: Vec::with_capacity(max_browse_continuation_points),
            diagnostics: SessionDiagnostics::new(),
            last_session_id: 0,
        }
    }

    pub fn new(server_state: &ServerState) -> Session {
        let max_publish_requests = MAX_DEFAULT_PUBLISH_REQUEST_QUEUE_SIZE;
        let max_browse_continuation_points = super::constants::MAX_BROWSE_CONTINUATION_POINTS;
        Session {
            subscriptions: Subscriptions::new(max_publish_requests, PUBLISH_REQUEST_TIMEOUT),
            session_id: NodeId::null(),
            activated: false,
            terminate_session: false,
            terminated: false,
            terminated_at: UTC::now(),
            client_certificate: ByteString::null(),
            security_policy_uri: String::new(),
            authentication_token: NodeId::null(),
            secure_channel: SecureChannel::new(server_state.certificate_store.clone()),
            session_nonce: ByteString::null(),
            session_timeout: 0f64,
            user_identity: None,
            max_request_message_size: 0,
            max_response_message_size: 0,
            endpoint_url: UAString::null(),
            max_browse_continuation_points,
            browse_continuation_points: Vec::with_capacity(max_browse_continuation_points),
            diagnostics: SessionDiagnostics::new(),
            last_session_id: 0,
        }
    }

    pub fn terminated(&mut self) {
        self.terminated = true;
        self.terminated_at = UTC::now();
    }

    pub fn next_session_id(&mut self) -> NodeId {
        self.last_session_id += 1;
        NodeId::new(1, self.last_session_id as u64)
    }

    pub fn diagnostics(&self) -> &SessionDiagnostics {
        &self.diagnostics
    }

    pub fn enqueue_publish_request(&mut self, server_state: &ServerState, request_id: UInt32, request: PublishRequest) -> Result<(), SupportedMessage> {
        let address_space = server_state.address_space.lock().unwrap();
        self.subscriptions.enqueue_publish_request(&address_space, request_id, request)
    }

    pub fn tick_subscriptions(&mut self, server_state: &ServerState, receive_publish_request: bool) -> Result<(), StatusCode> {
        let address_space = server_state.address_space.lock().unwrap();
        self.subscriptions.tick(receive_publish_request, &address_space)
    }

    /// Iterates through the existing queued publish requests and creates a timeout
    /// publish response any that have expired.
    pub fn expire_stale_publish_requests(&mut self, now: &DateTime<UTC>) {
        self.subscriptions.expire_stale_publish_requests(now);
    }

    pub fn add_browse_continuation_point(&mut self, continuation_point: BrowseContinuationPoint) {
        self.browse_continuation_points.push(continuation_point);
        while self.browse_continuation_points.len() > self.max_browse_continuation_points {
            let _ = self.browse_continuation_points.remove(0);
        }
    }

    /// Find a continuation point by id. If the continuation point is out of date is removed and None
    /// is returned.
    pub fn find_browse_continuation_point(&self, id: &ByteString) -> Option<BrowseContinuationPoint> {
        let continuation_point = self.browse_continuation_points.iter().find(|continuation_point| {
            continuation_point.id.eq(id)
        });
        if let Some(continuation_point) = continuation_point {
            Some(continuation_point.clone())
        } else {
            None
        }
    }

    pub fn remove_expired_browse_continuation_points(&mut self, address_space: &AddressSpace) {
        self.browse_continuation_points.retain(|continuation_point| {
            continuation_point.is_valid_browse_continuation_point(address_space)
        });
    }

    pub fn remove_browse_continuation_point(&mut self, continuation_point_id: &ByteString) {
        self.browse_continuation_points.retain(|continuation_point| {
            !continuation_point.id.eq(continuation_point_id)
        });
    }

    /// Remove all the specified continuation points by id
    pub fn remove_browse_continuation_points(&mut self, continuation_points: &[ByteString]) {
        use std::collections::HashSet;
        // Turn the supplied slice into a set
        let continuation_points_set: HashSet<ByteString> = continuation_points.iter().cloned().collect();
        // Now remove any continuation points that are part of that set
        self.browse_continuation_points.retain(|continuation_point| {
            !continuation_points_set.contains(&continuation_point.id)
        });
    }
}
