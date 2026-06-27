#[derive(Debug, Clone, PartialEq, Eq)]
enum CompactAdExpr<'a> {
    Scratch(usize),
    Call(CompactAdCall<'a>),
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CompactAdCall<'a> {
    name: &'a str,
    args: Vec<&'a str>,
}

pub(crate) fn lower_scaled_rhs_multiply(
    target_index: usize,
    source: usize,
    output_scale: &str,
    value: &str,
) -> Option<String> {
    let CompactAdExpr::Call(call) = parse_ad_expr(value) else {
        return None;
    };

    match call.name {
        "add" if call.args.len() == 2 => Some(format!(
            "scratch.store_mul_add_scaled_inputs_rhs({target_index}, {source}, {}, {output_scale}, {}, {output_scale});",
            call.args[0], call.args[1]
        )),
        "sub" if call.args.len() == 2 => Some(format!(
            "scratch.store_mul_sub_scaled_inputs_rhs({target_index}, {source}, {}, {output_scale}, {}, {output_scale});",
            call.args[0], call.args[1]
        )),
        "add_scaled_inputs" if call.args.len() == 4 => Some(format!(
            "scratch.store_mul_add_scaled_inputs_rhs({target_index}, {source}, {}, {}, {}, {});",
            call.args[0],
            scale_product(call.args[1], output_scale),
            call.args[2],
            scale_product(call.args[3], output_scale)
        )),
        "sub_scaled_inputs" if call.args.len() == 4 => Some(format!(
            "scratch.store_mul_sub_scaled_inputs_rhs({target_index}, {source}, {}, {}, {}, {});",
            call.args[0],
            scale_product(call.args[1], output_scale),
            call.args[2],
            scale_product(call.args[3], output_scale)
        )),
        "add_scaled_inputs3" if call.args.len() == 6 => Some(format!(
            "scratch.store_mul_add_scaled_inputs3_offset_rhs({target_index}, {source}, {}, {}, {}, {}, {}, {}, 0.0);",
            call.args[0],
            scale_product(call.args[1], output_scale),
            call.args[2],
            scale_product(call.args[3], output_scale),
            call.args[4],
            scale_product(call.args[5], output_scale)
        )),
        "add_scaled_inputs3_offset" if call.args.len() == 7 => Some(format!(
            "scratch.store_mul_add_scaled_inputs3_offset_rhs({target_index}, {source}, {}, {}, {}, {}, {}, {}, {});",
            call.args[0],
            scale_product(call.args[1], output_scale),
            call.args[2],
            scale_product(call.args[3], output_scale),
            call.args[4],
            scale_product(call.args[5], output_scale),
            scale_product(call.args[6], output_scale)
        )),
        "add_scaled_product" if call.args.len() == 5 => Some(format!(
            "scratch.store_mul_add_scaled_product_rhs({target_index}, {source}, {}, {}, {}, {}, {});",
            call.args[0],
            scale_product(call.args[1], output_scale),
            call.args[2],
            call.args[3],
            scale_product(call.args[4], output_scale)
        )),
        "add_scaled_sub_value_product" if call.args.len() == 6 => Some(format!(
            "scratch.store_mul_add_scaled_sub_value_product_rhs({target_index}, {source}, {}, {}, {}, {}, {}, {});",
            call.args[0],
            call.args[1],
            scale_product(call.args[2], output_scale),
            call.args[3],
            call.args[4],
            scale_product(call.args[5], output_scale)
        )),
        "add_scaled_inputs4" if call.args.len() == 8 => {
            let first_index = scratch_ad_value_index(call.args[0]);
            let second_index = scratch_ad_value_index(call.args[2]);
            let third_index = scratch_ad_value_index(call.args[4]);
            let fourth_index = scratch_ad_value_index(call.args[6]);
            match (first_index, second_index, third_index, fourth_index) {
                (Some(first), Some(second), Some(third), Some(fourth)) => Some(format!(
                    "scratch.store_mul_add_scaled_inputs4_indices_rhs({target_index}, {source}, {first}, {}, {second}, {}, {third}, {}, {fourth}, {});",
                    scale_product(call.args[1], output_scale),
                    scale_product(call.args[3], output_scale),
                    scale_product(call.args[5], output_scale),
                    scale_product(call.args[7], output_scale)
                )),
                _ => Some(format!(
                    "scratch.store_mul_add_scaled_inputs4_rhs({target_index}, {source}, {}, {}, {}, {}, {}, {}, {}, {});",
                    call.args[0],
                    scale_product(call.args[1], output_scale),
                    call.args[2],
                    scale_product(call.args[3], output_scale),
                    call.args[4],
                    scale_product(call.args[5], output_scale),
                    call.args[6],
                    scale_product(call.args[7], output_scale)
                )),
            }
        }
        "add_scaled_products" if call.args.len() == 6 => {
            let left_product_left_index = scratch_ad_value_index(call.args[0]);
            let left_product_right_index = scratch_ad_value_index(call.args[1]);
            let right_product_left_index = scratch_ad_value_index(call.args[3]);
            let right_product_right_index = scratch_ad_value_index(call.args[4]);
            match (
                left_product_left_index,
                left_product_right_index,
                right_product_left_index,
                right_product_right_index,
            ) {
                (
                    Some(left_product_left),
                    Some(left_product_right),
                    Some(right_product_left),
                    Some(right_product_right),
                ) => Some(format!(
                    "scratch.store_mul_add_scaled_products_indices_rhs({target_index}, {source}, {left_product_left}, {left_product_right}, {}, {right_product_left}, {right_product_right}, {});",
                    scale_product(call.args[2], output_scale),
                    scale_product(call.args[5], output_scale)
                )),
                _ => Some(format!(
                    "scratch.store_mul_add_scaled_products_rhs({target_index}, {source}, {}, {}, {}, {}, {}, {});",
                    call.args[0],
                    call.args[1],
                    scale_product(call.args[2], output_scale),
                    call.args[3],
                    call.args[4],
                    scale_product(call.args[5], output_scale)
                )),
            }
        }
        "add_scaled_products3" if call.args.len() == 9 => {
            let first_left_index = scratch_ad_value_index(call.args[0]);
            let first_right_index = scratch_ad_value_index(call.args[1]);
            let second_left_index = scratch_ad_value_index(call.args[3]);
            let second_right_index = scratch_ad_value_index(call.args[4]);
            let third_left_index = scratch_ad_value_index(call.args[6]);
            let third_right_index = scratch_ad_value_index(call.args[7]);
            match (
                first_left_index,
                first_right_index,
                second_left_index,
                second_right_index,
                third_left_index,
                third_right_index,
            ) {
                (
                    Some(first_left),
                    Some(first_right),
                    Some(second_left),
                    Some(second_right),
                    Some(third_left),
                    Some(third_right),
                ) => Some(format!(
                    "scratch.store_mul_add_scaled_products3_indices_rhs({target_index}, {source}, {first_left}, {first_right}, {}, {second_left}, {second_right}, {}, {third_left}, {third_right}, {});",
                    scale_product(call.args[2], output_scale),
                    scale_product(call.args[5], output_scale),
                    scale_product(call.args[8], output_scale)
                )),
                _ => Some(format!(
                    "scratch.store_mul_add_scaled_products3_rhs({target_index}, {source}, {}, {}, {}, {}, {}, {}, {}, {}, {});",
                    call.args[0],
                    call.args[1],
                    scale_product(call.args[2], output_scale),
                    call.args[3],
                    call.args[4],
                    scale_product(call.args[5], output_scale),
                    call.args[6],
                    call.args[7],
                    scale_product(call.args[8], output_scale)
                )),
            }
        }
        "scale_offset" if call.args.len() == 3 => {
            if let CompactAdExpr::Call(inner_call) = parse_ad_expr(call.args[0]) {
                if inner_call.name == "powf" && inner_call.args.len() == 2 {
                    if let Some(base_source) = scratch_ad_value_index(inner_call.args[0]) {
                        return Some(format!(
                            "scratch.store_mul_scaled_powf_scale_offset_rhs({target_index}, {source}, {output_scale}, {base_source}, {}, {}, {});",
                            inner_call.args[1], call.args[1], call.args[2]
                        ));
                    }
                }
            }
            scratch_ad_value_index(call.args[0]).map(|right_source| {
                format!(
                    "scratch.store_mul_scale_offset_rhs({target_index}, {source}, {right_source}, {}, {});",
                    scale_product(call.args[1], output_scale),
                    scale_product(call.args[2], output_scale)
                )
            })
        }
        "div" if call.args.len() == 2 => Some(format!(
            "scratch.store_mul_div_scaled_inputs_rhs({target_index}, {source}, {}, {output_scale}, {}, 1.0);",
            call.args[0], call.args[1]
        )),
        "pow" if call.args.len() == 2 => Some(format!(
            "scratch.store_mul_scaled_pow_ad_rhs({target_index}, {source}, {output_scale}, {}, {});",
            call.args[0], call.args[1]
        )),
        "sqrt_scaled_input" if call.args.len() == 2 => scratch_ad_value_index(call.args[0]).map(
            |value_source| {
                format!(
                    "scratch.store_mul_scaled_sqrt_scaled_input_rhs({target_index}, {source}, {output_scale}, {value_source}, {});",
                    call.args[1]
                )
            },
        ),
        "exp_scaled_input" if call.args.len() == 2 => {
            if let CompactAdExpr::Call(inner_call) = parse_ad_expr(call.args[0]) {
                if inner_call.name == "ln" && inner_call.args.len() == 1 {
                    return Some(format!(
                        "scratch.store_mul_scaled_exp_ln_input_rhs({target_index}, {source}, {output_scale}, {}, {});",
                        inner_call.args[0], call.args[1]
                    ));
                }
            }
            scratch_ad_value_index(call.args[0]).map(|value_source| {
                format!(
                    "scratch.store_mul_scaled_exp_scaled_input_rhs({target_index}, {source}, {output_scale}, {value_source}, {});",
                    call.args[1]
                )
            })
        }
        _ => None,
    }
}

fn parse_ad_expr(value: &str) -> CompactAdExpr<'_> {
    let value = value.trim();
    if let Some(index) = scratch_ad_value_index(value) {
        return CompactAdExpr::Scratch(index);
    }
    if let Some(call) = ad_call(value) {
        return CompactAdExpr::Call(call);
    }
    CompactAdExpr::Other
}

fn scratch_ad_value_index(value: &str) -> Option<usize> {
    let value = value.trim();
    value
        .strip_prefix("scratch.ad_value(")
        .or_else(|| value.strip_prefix("s.ad_value("))?
        .strip_suffix(')')?
        .parse()
        .ok()
}

fn ad_call(value: &str) -> Option<CompactAdCall<'_>> {
    let value = value.trim();
    let tail = value
        .strip_prefix("AdValue::")
        .or_else(|| value.strip_prefix("A::"))?;
    let open = tail.find('(')?;
    let name = tail[..open].trim();
    if name.is_empty() {
        return None;
    }
    let inner = tail[open + 1..].strip_suffix(')')?;
    Some(CompactAdCall {
        name,
        args: split_top_level_args(inner)?,
    })
}

fn split_top_level_args(input: &str) -> Option<Vec<&str>> {
    let mut args = Vec::new();
    let mut start = 0;
    let mut paren_depth = 0usize;
    let mut bracket_depth = 0usize;
    let mut brace_depth = 0usize;

    for (index, ch) in input.char_indices() {
        match ch {
            '(' => paren_depth = paren_depth.checked_add(1)?,
            ')' => paren_depth = paren_depth.checked_sub(1)?,
            '[' => bracket_depth = bracket_depth.checked_add(1)?,
            ']' => bracket_depth = bracket_depth.checked_sub(1)?,
            '{' => brace_depth = brace_depth.checked_add(1)?,
            '}' => brace_depth = brace_depth.checked_sub(1)?,
            ',' if paren_depth == 0 && bracket_depth == 0 && brace_depth == 0 => {
                args.push(input[start..index].trim());
                start = index + ch.len_utf8();
            }
            _ => {}
        }
    }

    if paren_depth != 0 || bracket_depth != 0 || brace_depth != 0 {
        return None;
    }
    args.push(input[start..].trim());
    Some(args)
}

fn scale_product(left: &str, right: &str) -> String {
    if left == "1.0" {
        right.to_string()
    } else if right == "1.0" {
        left.to_string()
    } else {
        format!("(({left}) * ({right}))")
    }
}
