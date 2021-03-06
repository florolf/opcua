// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateEventDetails {
    pub node_id: NodeId,
    pub perform_insert_replace: PerformUpdateType,
    pub filter: EventFilter,
    pub event_data: Option<Vec<HistoryEventFieldList>>,
}

impl BinaryEncoder<UpdateEventDetails> for UpdateEventDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.perform_insert_replace.byte_len();
        size += self.filter.byte_len();
        size += byte_len_array(&self.event_data);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.perform_insert_replace.encode(stream)?;
        size += self.filter.encode(stream)?;
        size += write_array(stream, &self.event_data)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream)?;
        let perform_insert_replace = PerformUpdateType::decode(stream)?;
        let filter = EventFilter::decode(stream)?;
        let event_data: Option<Vec<HistoryEventFieldList>> = read_array(stream)?;
        Ok(UpdateEventDetails {
            node_id: node_id,
            perform_insert_replace: perform_insert_replace,
            filter: filter,
            event_data: event_data,
        })
    }
}
