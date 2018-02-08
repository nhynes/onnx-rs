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
        $proto.$setter(RepeatedField::from_vec($val.into_iter().map(|v| v.into()).collect()))
    }
}

pub fn make_model(graph: GraphProto, opset_imports: Vec<OperatorSetIdProto>) -> ModelProto {
    let mut model_proto = ModelProto::new();
    model_proto.set_ir_version(Version::IR_VERSION.value() as i64);
    model_proto.set_graph(graph);
    model_proto.set_opset_import(RepeatedField::from_vec(if opset_imports.len() > 0 {
        opset_imports
    } else {
        vec![make_opsetid(None: Option<String>, 3)]
    }));
    model_proto
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

pub fn make_opsetid<S: Into<String>, T: Into<i64>>(
    domain: Option<S>,
    version: T,
) -> OperatorSetIdProto {
    let mut opsetid_proto = OperatorSetIdProto::new();
    set_optional!(opsetid_proto.set_domain(domain));
    opsetid_proto.set_version(version.into());
    opsetid_proto
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
