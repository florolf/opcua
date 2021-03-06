// This file was autogenerated from Opc.Ua.Types.bsd.xml
// DO NOT EDIT THIS FILE

use std::io::{Read, Write};

#[allow(unused_imports)]
use types::*;
#[allow(unused_imports)]
use services::*;

#[derive(Debug, Clone, PartialEq)]
pub struct UpdateDataDetails {
    pub node_id: NodeId,
    pub perform_insert_replace: PerformUpdateType,
    pub update_values: Option<Vec<DataValue>>,
}

impl BinaryEncoder<UpdateDataDetails> for UpdateDataDetails {
    fn byte_len(&self) -> usize {
        let mut size = 0;
        size += self.node_id.byte_len();
        size += self.perform_insert_replace.byte_len();
        size += byte_len_array(&self.update_values);
        size
    }

    #[allow(unused_variables)]
    fn encode<S: Write>(&self, stream: &mut S) -> EncodingResult<usize> {
        let mut size = 0;
        size += self.node_id.encode(stream)?;
        size += self.perform_insert_replace.encode(stream)?;
        size += write_array(stream, &self.update_values)?;
        Ok(size)
    }

    #[allow(unused_variables)]
    fn decode<S: Read>(stream: &mut S) -> EncodingResult<Self> {
        let node_id = NodeId::decode(stream)?;
        let perform_insert_replace = PerformUpdateType::decode(stream)?;
        let update_values: Option<Vec<DataValue>> = read_array(stream)?;
        Ok(UpdateDataDetails {
            node_id: node_id,
            perform_insert_replace: perform_insert_replace,
            update_values: update_values,
        })
    }
}
