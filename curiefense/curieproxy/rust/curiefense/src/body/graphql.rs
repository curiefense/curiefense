use async_graphql_parser::{
    parse_query,
    types::{Directive, DocumentOperations, OperationDefinition, Selection, SelectionSet},
    Positioned,
};

use crate::{config::utils::DataSource, requestfields::RequestField};

fn insert_directive(args: &mut RequestField, prefix: String, dir: Directive) {
    for (n, v) in dir.arguments {
        let prefix = prefix.clone() + "-" + &dir.name.node + "-" + &n.node;
        args.add(prefix, DataSource::FromBody, v.node.to_string());
    }
}

fn insert_dirsels(
    args: &mut RequestField,
    prefix: &str,
    directives: Vec<Positioned<Directive>>,
    mselections: Option<Positioned<SelectionSet>>,
) -> bool {
    let mut o = false;
    for (n, d) in directives.into_iter().enumerate() {
        o = true;
        insert_directive(args, format!("{}-d{}", prefix, n), d.node);
    }
    if let Some(selections) = mselections {
        for (n, s) in selections.node.items.into_iter().enumerate() {
            o = true;
            insert_selection(args, format!("{}-s{}", prefix, n), s.node);
        }
    }
    o
}

fn insert_selection(args: &mut RequestField, prefix: String, sel: Selection) {
    match sel {
        Selection::Field(pfield) => {
            let field = pfield.node;
            let mut traced = false;
            let nprefix = prefix.to_string() + "-" + &field.name.node;
            if let Some(alias) = field.alias {
                traced = true;
                args.add(
                    nprefix.to_string() + "-alias",
                    DataSource::FromBody,
                    alias.node.to_string(),
                );
            }
            for (k, v) in field.arguments {
                traced = true;
                args.add(
                    nprefix.to_string() + "-" + &k.node,
                    DataSource::FromBody,
                    v.node.to_string(),
                );
            }
            traced |= insert_dirsels(args, &nprefix, field.directives, Some(field.selection_set));
            if !traced {
                args.add(prefix.clone(), DataSource::FromBody, field.name.node.to_string());
            }
        }
        Selection::FragmentSpread(fsp) => {
            let frag = fsp.node;
            let traced = insert_dirsels(args, &prefix, frag.directives, None);
            if !traced {
                args.add(
                    prefix.to_string() + "-frag",
                    DataSource::FromBody,
                    frag.fragment_name.node.to_string(),
                );
            }
        }
        Selection::InlineFragment(pinline) => {
            let inline = pinline.node;
            insert_dirsels(args, &prefix, inline.directives, Some(inline.selection_set));
        }
    }
}

fn insert_operation(args: &mut RequestField, mprefix: Option<&str>, pod: Positioned<OperationDefinition>) {
    let mut prefix = "gdir".to_string();
    let od = pod.node;
    if let Some(p) = mprefix {
        prefix += "-";
        prefix += p;
    }
    for pvardef in od.variable_definitions {
        let vardef = pvardef.node;
        let varprefix = prefix.clone() + "-" + &vardef.name.node;
        if let Some(cval) = vardef.default_value {
            args.add(varprefix.clone() + "-defvalue", DataSource::FromBody, cval.to_string());
        }
        insert_dirsels(args, &varprefix, vardef.directives, None);
    }
    insert_dirsels(args, &prefix, od.directives, Some(od.selection_set));
}

pub fn graphql_body(args: &mut RequestField, body: &[u8]) -> Result<(), String> {
    let body_utf8 = std::str::from_utf8(body).map_err(|rr| rr.to_string())?;
    let document = parse_query(body_utf8).map_err(|rr| rr.to_string())?;
    for (nm, pdef) in document.fragments {
        let basename = "gfrag-".to_string() + &nm;
        insert_dirsels(args, &basename, pdef.node.directives, Some(pdef.node.selection_set));
    }

    match document.operations {
        DocumentOperations::Single(opdef) => insert_operation(args, None, opdef),
        DocumentOperations::Multiple(opdefs) => {
            for (n, op) in opdefs {
                insert_operation(args, Some(&n), op);
            }
        }
    }

    Ok(())
}
