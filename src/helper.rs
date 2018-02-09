use std::collections::HashMap;

use proto::*;
use protobuf::{ProtobufEnum, RepeatedField};

macro_rules! set_optional {
    ($proto: ident . $setter: ident ( $val: ident ) ) => {
        if let Some($val) = $val {
            $proto.$setter($val.into())
        }
    }
}

macro_rules! set_repeated {
    ($proto: ident . $setter: ident ( $val: ident ) ) =>  {
        $proto.$setter(RepeatedField::from_vec($val))
    };
    ($proto: ident . $setter: ident ( $val: ident .into() ) ) =>  {
        $proto.$setter(RepeatedField::from_vec($val.into_iter().map(Into::into).collect()))
    }
}

pub fn make_model<S: Into<String>, T: Into<i64>>(
    graph: GraphProto,
    opset_imports: Vec<OperatorSetIdProto>,
    domain: Option<String>,
    model_version: Option<T>,
    producer_name: Option<S>,
    producer_version: Option<S>,
    doc_string: Option<S>,
    metadata: Option<HashMap<String, String>>,
) -> ModelProto {
    let mut model_proto = ModelProto::new();
    model_proto.set_ir_version(Version::IR_VERSION.value() as i64);
    model_proto.set_graph(graph);
    model_proto.set_opset_import(RepeatedField::from_vec(if opset_imports.len() > 0 {
        opset_imports
    } else {
        vec![make_opsetid(None: Option<String>, 3)]
    }));
    set_optional!(model_proto.set_domain(domain));
    set_optional!(model_proto.set_model_version(model_version));
    set_optional!(model_proto.set_producer_name(producer_name));
    set_optional!(model_proto.set_producer_version(producer_version));
    set_optional!(model_proto.set_doc_string(doc_string));
    if let Some(metadata) = metadata {
        model_proto.set_metadata_props(RepeatedField::from_vec(
            metadata
                .into_iter()
                .map(|(k, v)| {
                    let mut ss_proto = StringStringEntryProto::new();
                    ss_proto.set_key(k.into());
                    ss_proto.set_value(v.into());
                    ss_proto
                })
                .collect(),
        ));
    }
    model_proto
}

pub fn make_opsetid<S: Into<String>, T: Into<i64>>(
    domain: Option<S>,
    version: T,
) -> OperatorSetIdProto {
    let mut opsetid_proto = OperatorSetIdProto::new();
    set_optional!(opsetid_proto.set_domain(domain));
    opsetid_proto.set_version(version.into());
    opsetid_proto
}

pub fn make_graph<S: Into<String>>(
    nodes: Vec<NodeProto>,
    name: S,
    inputs: Vec<ValueInfoProto>,
    outputs: Vec<ValueInfoProto>,
    initializer: Vec<TensorProto>,
    doc_string: Option<S>,
) -> GraphProto {
    let mut graph_proto = GraphProto::new();
    graph_proto.set_name(name.into());
    set_repeated!(graph_proto.set_node(nodes));
    set_repeated!(graph_proto.set_input(inputs));
    set_repeated!(graph_proto.set_output(outputs));
    set_repeated!(graph_proto.set_initializer(initializer));
    set_optional!(graph_proto.set_doc_string(doc_string));
    graph_proto
}

pub fn make_node<S: Into<String>>(
    op_type: Option<S>,
    inputs: Vec<S>,
    outputs: Vec<S>,
    name: Option<S>,
    doc_string: Option<S>,
    domain: Option<S>,
    attributes: Vec<AttributeProto>,
) -> NodeProto {
    let mut node_proto = NodeProto::new();
    set_optional!(node_proto.set_op_type(op_type));
    set_repeated!(node_proto.set_input(inputs.into()));
    set_repeated!(node_proto.set_output(outputs.into()));
    set_optional!(node_proto.set_name(name));
    set_optional!(node_proto.set_domain(domain));
    set_optional!(node_proto.set_doc_string(doc_string));
    set_repeated!(node_proto.set_attribute(attributes));
    node_proto
}

pub enum Attribute<S> {
    Float(f32),
    Floats(Vec<f32>),
    Int(i64),
    Ints(Vec<i64>),
    String(S),
    Strings(Vec<S>),
    Tensor(TensorProto),
    Tensors(Vec<TensorProto>),
    Graph(GraphProto),
    Graphs(Vec<GraphProto>),
}

pub fn make_attribute<S: Into<String>, U: Into<Vec<u8>>>(
    name: S,
    attribute: Attribute<U>,
) -> AttributeProto {
    let mut attr_proto = AttributeProto::new();
    attr_proto.set_name(name.into());
    match attribute {
        Attribute::Float(val) => {
            attr_proto.set_f(val);
            attr_proto.set_field_type(AttributeProto_AttributeType::FLOAT);
        }
        Attribute::Floats(vals) => {
            attr_proto.set_floats(vals);
            attr_proto.set_field_type(AttributeProto_AttributeType::FLOATS);
        }
        Attribute::Int(val) => {
            attr_proto.set_i(val);
            attr_proto.set_field_type(AttributeProto_AttributeType::INT);
        }
        Attribute::Ints(vals) => {
            attr_proto.set_ints(vals);
            attr_proto.set_field_type(AttributeProto_AttributeType::INTS);
        }
        Attribute::String(val) => {
            attr_proto.set_s(val.into());
            attr_proto.set_field_type(AttributeProto_AttributeType::STRING);
        }
        Attribute::Strings(vals) => {
            attr_proto.set_strings(vals.into_iter().map(Into::into).collect());
            attr_proto.set_field_type(AttributeProto_AttributeType::STRINGS);
        }
        Attribute::Graph(val) => {
            attr_proto.set_g(val);
            attr_proto.set_field_type(AttributeProto_AttributeType::GRAPH);
        }
        Attribute::Graphs(vals) => {
            set_repeated!(attr_proto.set_graphs(vals));
            attr_proto.set_field_type(AttributeProto_AttributeType::GRAPHS);
        }
        Attribute::Tensor(val) => {
            attr_proto.set_t(val);
            attr_proto.set_field_type(AttributeProto_AttributeType::TENSOR);
        }
        Attribute::Tensors(vals) => {
            set_repeated!(attr_proto.set_tensors(vals));
            attr_proto.set_field_type(AttributeProto_AttributeType::TENSORS);
        }
    };
    attr_proto
}

pub enum TensorValue {
    Float(Vec<f32>),
    UInt8(Vec<u8>),
    Int8(Vec<i8>),
    UInt16(Vec<u16>),
    Int16(Vec<i16>),
    Int32(Vec<i32>),
    Int64(Vec<i64>),
    String(Vec<String>),
    Bool(Vec<bool>),
    Double(Vec<f64>),
    UInt32(Vec<u32>),
    UInt64(Vec<u64>),
}

macro_rules! set_tensor_data {
    ($proto: ident, $vals: ident $(| $type: ident $proto_type: ident $setter: ident)+) => {
        match $vals {
            TensorValue::Bool(vals) => {
                $proto.set_int32_data(vals.into_iter().map(|v| if v { 1 } else { 0 }).collect());
                $proto.set_data_type(TensorProto_DataType::BOOL);
            }
            $(TensorValue::$type(vals) => {
                $proto.$setter(vals.into_iter().map(Into::into).collect());
                $proto.set_data_type(TensorProto_DataType::$proto_type);
            })+
        };
    }
}

pub fn make_tensor<S: Into<String>>(
    name: Option<S>,
    dims: Vec<i64>,
    vals: TensorValue,
) -> TensorProto {
    let mut tensor_proto = TensorProto::new();
    tensor_proto.set_dims(dims);
    set_optional!(tensor_proto.set_name(name));
    set_tensor_data!(tensor_proto, vals
        | Float   FLOAT   set_float_data
        | UInt8   UINT8   set_int32_data
        | Int8    INT8    set_int32_data
        | UInt16  UINT16  set_int32_data
        | Int16   INT16   set_int32_data
        | Int32   INT32   set_int32_data
        | String  STRING  set_string_data
        | UInt32  UINT32  set_uint64_data
        | UInt64   UINT64  set_uint64_data
        | Int64   INT64   set_int64_data
        | Double  DOUBLE  set_double_data
        // | Bool    BOOL    set_int32_data // no from -> special-cased
    );
    tensor_proto
}

pub enum Dimension {
    Value(i64),
    Param(String),
}

// impl<T: Into<i64>> From<T> for Dimension {
//     fn from(val: T) -> Self {
//         Dimension::Value(val.into())
//     }
// }
//
// impl<T: Into<String>> From<T> for Dimension {
//     fn from(val: T) -> Self {
//         Dimension::Param(val.into())
//     }
// }

pub fn make_tensor_value_info<S: Into<String>>(
    name: S,
    elem_type: TensorProto_DataType,
    shape: Vec<Dimension>,
    doc_string: Option<S>,
) -> ValueInfoProto {
    let mut tensor_shape_proto = TensorShapeProto::new();
    tensor_shape_proto.set_dim(
        shape
            .into_iter()
            .map(|s| {
                let mut dim = TensorShapeProto_Dimension::new();
                match s {
                    Dimension::Value(v) => dim.set_dim_value(v),
                    Dimension::Param(p) => dim.set_dim_param(p),
                };
                dim
            })
            .collect(),
    );
    let mut tensor_type_proto = TypeProto_Tensor::new();
    tensor_type_proto.set_elem_type(elem_type);
    tensor_type_proto.set_shape(tensor_shape_proto);

    let mut type_proto = TypeProto::new();
    type_proto.set_tensor_type(tensor_type_proto);

    let mut value_info_proto = ValueInfoProto::new();
    value_info_proto.set_name(name.into());
    value_info_proto.set_field_type(type_proto);
    set_optional!(value_info_proto.set_doc_string(doc_string));
    value_info_proto
}
