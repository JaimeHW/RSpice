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
        "add" if call.args.len() == 2 => Some(lower_mul_scaled_inputs_rhs(
            "store_mul_add_scaled_inputs_rhs",
            target_index,
            source,
            call.args[0],
            output_scale,
            call.args[1],
            output_scale,
        )),
        "sub" if call.args.len() == 2 => Some(lower_mul_scaled_inputs_rhs(
            "store_mul_sub_scaled_inputs_rhs",
            target_index,
            source,
            call.args[0],
            output_scale,
            call.args[1],
            output_scale,
        )),
        "add_scaled_inputs" if call.args.len() == 4 => Some(lower_mul_scaled_inputs_rhs(
            "store_mul_add_scaled_inputs_rhs",
            target_index,
            source,
            call.args[0],
            &scale_product(call.args[1], output_scale),
            call.args[2],
            &scale_product(call.args[3], output_scale),
        )),
        "sub_scaled_inputs" if call.args.len() == 4 => Some(lower_mul_scaled_inputs_rhs(
            "store_mul_sub_scaled_inputs_rhs",
            target_index,
            source,
            call.args[0],
            &scale_product(call.args[1], output_scale),
            call.args[2],
            &scale_product(call.args[3], output_scale),
        )),
        "add_scaled_inputs3" if call.args.len() == 6 => Some(
            lower_mul_add_scaled_inputs3_offset_rhs(
                target_index,
                source,
                call.args[0],
                &scale_product(call.args[1], output_scale),
                call.args[2],
                &scale_product(call.args[3], output_scale),
                call.args[4],
                &scale_product(call.args[5], output_scale),
                "0.0",
            ),
        ),
        "add_scaled_inputs3_offset" if call.args.len() == 7 => Some(
            lower_mul_add_scaled_inputs3_offset_rhs(
                target_index,
                source,
                call.args[0],
                &scale_product(call.args[1], output_scale),
                call.args[2],
                &scale_product(call.args[3], output_scale),
                call.args[4],
                &scale_product(call.args[5], output_scale),
                &scale_product(call.args[6], output_scale),
            ),
        ),
        "add_scaled_product" if call.args.len() == 5 => {
            let value_scale = scale_product(call.args[1], output_scale);
            let product_scale = scale_product(call.args[4], output_scale);
            if let Some(line) = lower_mul_add_scaled_product_value_term(
                target_index,
                source,
                call.args[0],
                &value_scale,
                call.args[2],
                call.args[3],
                &product_scale,
            ) {
                return Some(line);
            }
            Some(lower_mul_add_scaled_product_rhs(
                target_index,
                source,
                call.args[0],
                &value_scale,
                call.args[2],
                call.args[3],
                &product_scale,
            ))
        }
        "add_scaled_sub_value_product" if call.args.len() == 6 => {
            Some(lower_mul_add_scaled_sub_value_product_rhs(
                target_index,
                source,
                call.args[0],
                call.args[1],
                &scale_product(call.args[2], output_scale),
                call.args[3],
                call.args[4],
                &scale_product(call.args[5], output_scale),
            ))
        }
        "add_scaled_inputs4" if call.args.len() == 8 => {
            Some(lower_mul_add_scaled_inputs4_rhs(
                target_index,
                source,
                call.args[0],
                &scale_product(call.args[1], output_scale),
                call.args[2],
                &scale_product(call.args[3], output_scale),
                call.args[4],
                &scale_product(call.args[5], output_scale),
                call.args[6],
                &scale_product(call.args[7], output_scale),
            ))
        }
        "add_scaled_products" if call.args.len() == 6 => {
            Some(lower_mul_add_scaled_products_rhs(
                target_index,
                source,
                call.args[0],
                call.args[1],
                &scale_product(call.args[2], output_scale),
                call.args[3],
                call.args[4],
                &scale_product(call.args[5], output_scale),
            ))
        }
        "add_scaled_products3" if call.args.len() == 9 => {
            Some(lower_mul_add_scaled_products3_rhs(
                target_index,
                source,
                call.args[0],
                call.args[1],
                &scale_product(call.args[2], output_scale),
                call.args[3],
                call.args[4],
                &scale_product(call.args[5], output_scale),
                call.args[6],
                call.args[7],
                &scale_product(call.args[8], output_scale),
            ))
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
                if inner_call.name == "powi" && inner_call.args.len() == 2 {
                    if let Some(base_source) = scratch_ad_value_index(inner_call.args[0]) {
                        return Some(format!(
                            "scratch.store_mul_scaled_powi_scale_offset_rhs({target_index}, {source}, {output_scale}, {base_source}, {}, {}, {});",
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
        "div" if call.args.len() == 2 => Some(lower_mul_div_scaled_inputs_rhs(
            target_index,
            source,
            call.args[0],
            output_scale,
            call.args[1],
            "1.0",
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

fn lower_mul_scaled_inputs_rhs(
    base: &str,
    target_index: usize,
    source: usize,
    left: &str,
    left_scale: &str,
    right: &str,
    right_scale: &str,
) -> String {
    let (left_mask, left_arg) = index_or_ad_arg(left);
    let (right_mask, right_arg) = index_or_ad_arg(right);
    let mask = format!("{left_mask}{right_mask}");
    let helper = index_or_mixed_helper_name(base, &mask);
    format!(
        "scratch.{helper}({target_index}, {source}, {left_arg}, {left_scale}, {right_arg}, {right_scale});"
    )
}

fn lower_mul_add_scaled_inputs3_offset_rhs(
    target_index: usize,
    source: usize,
    first: &str,
    first_scale: &str,
    second: &str,
    second_scale: &str,
    third: &str,
    third_scale: &str,
    offset: &str,
) -> String {
    let (first_mask, first_arg) = index_or_ad_arg(first);
    let (second_mask, second_arg) = index_or_ad_arg(second);
    let (third_mask, third_arg) = index_or_ad_arg(third);
    let mask = format!("{first_mask}{second_mask}{third_mask}");
    let helper = index_or_mixed_helper_name("store_mul_add_scaled_inputs3_offset_rhs", &mask);
    format!(
        "scratch.{helper}({target_index}, {source}, {first_arg}, {first_scale}, {second_arg}, {second_scale}, {third_arg}, {third_scale}, {offset});"
    )
}

fn lower_mul_add_scaled_inputs4_rhs(
    target_index: usize,
    source: usize,
    first: &str,
    first_scale: &str,
    second: &str,
    second_scale: &str,
    third: &str,
    third_scale: &str,
    fourth: &str,
    fourth_scale: &str,
) -> String {
    let (first_mask, first_arg) = index_or_ad_arg(first);
    let (second_mask, second_arg) = index_or_ad_arg(second);
    let (third_mask, third_arg) = index_or_ad_arg(third);
    let (fourth_mask, fourth_arg) = index_or_ad_arg(fourth);
    let mask = format!("{first_mask}{second_mask}{third_mask}{fourth_mask}");
    let helper = mul_add_scaled_inputs4_rhs_helper_name(&mask);
    format!(
        "scratch.{helper}({target_index}, {source}, {first_arg}, {first_scale}, {second_arg}, {second_scale}, {third_arg}, {third_scale}, {fourth_arg}, {fourth_scale});"
    )
}

fn mul_add_scaled_inputs4_rhs_helper_name(mask: &str) -> String {
    if mask.bytes().all(|byte| byte == b'i') {
        "store_mul_add_scaled_inputs4_indices_rhs".to_string()
    } else {
        index_or_mixed_helper_name("store_mul_add_scaled_inputs4_rhs", mask)
    }
}

fn lower_mul_add_scaled_product_value_term(
    target_index: usize,
    source: usize,
    value: &str,
    value_scale: &str,
    product_left: &str,
    product_right: &str,
    product_scale: &str,
) -> Option<String> {
    let product_left = scratch_ad_value_index(product_left)?;
    let product_right = scratch_ad_value_index(product_right)?;
    let CompactAdExpr::Call(call) = parse_ad_expr(value) else {
        return None;
    };

    match call.name {
        "sqrt" if call.args.len() == 1 => {
            let value = scratch_ad_value_index(call.args[0])?;
            Some(format!(
                "scratch.store_mul_add_scaled_product_sqrt_value_rhs({target_index}, {source}, {value}, {value_scale}, {product_left}, {product_right}, {product_scale});"
            ))
        }
        "sub" if call.args.len() == 2 => {
            let value_left = scratch_ad_value_index(call.args[0])?;
            let value_right = scratch_ad_value_index(call.args[1])?;
            Some(format!(
                "scratch.store_mul_add_scaled_product_sub_value_rhs({target_index}, {source}, {value_left}, {value_right}, {value_scale}, {product_left}, {product_right}, {product_scale});"
            ))
        }
        _ => None,
    }
}

fn lower_mul_add_scaled_product_rhs(
    target_index: usize,
    source: usize,
    value: &str,
    value_scale: &str,
    product_left: &str,
    product_right: &str,
    product_scale: &str,
) -> String {
    let (value_mask, value_arg) = index_or_ad_arg(value);
    let (product_left_mask, product_left_arg) = index_or_ad_arg(product_left);
    let (product_right_mask, product_right_arg) = index_or_ad_arg(product_right);
    let mask = format!("{value_mask}{product_left_mask}{product_right_mask}");
    let helper = index_or_mixed_helper_name("store_mul_add_scaled_product_rhs", &mask);
    format!(
        "scratch.{helper}({target_index}, {source}, {value_arg}, {value_scale}, {product_left_arg}, {product_right_arg}, {product_scale});"
    )
}

fn lower_mul_add_scaled_sub_value_product_rhs(
    target_index: usize,
    source: usize,
    scalar: &str,
    subtrahend: &str,
    value_scale: &str,
    product_left: &str,
    product_right: &str,
    product_scale: &str,
) -> String {
    let (subtrahend_mask, subtrahend_arg) = index_or_ad_arg(subtrahend);
    let (product_left_mask, product_left_arg) = index_or_ad_arg(product_left);
    let (product_right_mask, product_right_arg) = index_or_ad_arg(product_right);
    let mask = format!("{subtrahend_mask}{product_left_mask}{product_right_mask}");
    let helper = index_or_mixed_helper_name("store_mul_add_scaled_sub_value_product_rhs", &mask);
    format!(
        "scratch.{helper}({target_index}, {source}, {scalar}, {subtrahend_arg}, {value_scale}, {product_left_arg}, {product_right_arg}, {product_scale});"
    )
}

fn lower_mul_add_scaled_products_rhs(
    target_index: usize,
    source: usize,
    left_product_left: &str,
    left_product_right: &str,
    left_scale: &str,
    right_product_left: &str,
    right_product_right: &str,
    right_scale: &str,
) -> String {
    let (left_product_left_mask, left_product_left_arg) = index_or_ad_arg(left_product_left);
    let (left_product_right_mask, left_product_right_arg) = index_or_ad_arg(left_product_right);
    let (right_product_left_mask, right_product_left_arg) = index_or_ad_arg(right_product_left);
    let (right_product_right_mask, right_product_right_arg) = index_or_ad_arg(right_product_right);
    let mask = format!(
        "{left_product_left_mask}{left_product_right_mask}{right_product_left_mask}{right_product_right_mask}"
    );
    let helper = mul_add_scaled_products_rhs_helper_name(&mask);
    format!(
        "scratch.{helper}({target_index}, {source}, {left_product_left_arg}, {left_product_right_arg}, {left_scale}, {right_product_left_arg}, {right_product_right_arg}, {right_scale});"
    )
}

fn mul_add_scaled_products_rhs_helper_name(mask: &str) -> String {
    if mask.bytes().all(|byte| byte == b'i') {
        "store_mul_add_scaled_products_indices_rhs".to_string()
    } else {
        index_or_mixed_helper_name("store_mul_add_scaled_products_rhs", mask)
    }
}

#[allow(clippy::too_many_arguments)]
fn lower_mul_add_scaled_products3_rhs(
    target_index: usize,
    source: usize,
    first_left: &str,
    first_right: &str,
    first_scale: &str,
    second_left: &str,
    second_right: &str,
    second_scale: &str,
    third_left: &str,
    third_right: &str,
    third_scale: &str,
) -> String {
    let (first_left_mask, first_left_arg) = index_or_ad_arg(first_left);
    let (first_right_mask, first_right_arg) = index_or_ad_arg(first_right);
    let (second_left_mask, second_left_arg) = index_or_ad_arg(second_left);
    let (second_right_mask, second_right_arg) = index_or_ad_arg(second_right);
    let (third_left_mask, third_left_arg) = index_or_ad_arg(third_left);
    let (third_right_mask, third_right_arg) = index_or_ad_arg(third_right);
    let mask = format!(
        "{first_left_mask}{first_right_mask}{second_left_mask}{second_right_mask}{third_left_mask}{third_right_mask}"
    );
    let helper = mul_add_scaled_products3_rhs_helper_name(&mask);
    format!(
        "scratch.{helper}({target_index}, {source}, {first_left_arg}, {first_right_arg}, {first_scale}, {second_left_arg}, {second_right_arg}, {second_scale}, {third_left_arg}, {third_right_arg}, {third_scale});"
    )
}

fn mul_add_scaled_products3_rhs_helper_name(mask: &str) -> String {
    if mask.bytes().all(|byte| byte == b'i') {
        "store_mul_add_scaled_products3_indices_rhs".to_string()
    } else {
        index_or_mixed_helper_name("store_mul_add_scaled_products3_rhs", mask)
    }
}

fn lower_mul_div_scaled_inputs_rhs(
    target_index: usize,
    source: usize,
    numerator: &str,
    numerator_scale: &str,
    denominator: &str,
    denominator_scale: &str,
) -> String {
    let (numerator_mask, numerator_arg) = index_or_ad_arg(numerator);
    let (denominator_mask, denominator_arg) = index_or_ad_arg(denominator);
    let mask = format!("i{numerator_mask}{denominator_mask}");
    let helper = index_or_mixed_helper_name("store_mul_div_scaled_inputs", &mask);
    format!(
        "scratch.{helper}({target_index}, {source}, {numerator_arg}, {numerator_scale}, {denominator_arg}, {denominator_scale});"
    )
}

fn index_or_ad_arg(value: &str) -> (char, String) {
    if let Some(index) = scratch_ad_value_index(value) {
        ('i', index.to_string())
    } else {
        ('a', value.to_string())
    }
}

fn index_or_mixed_helper_name(base: &str, mask: &str) -> String {
    if mask.bytes().all(|byte| byte == b'i') {
        format!("{base}_indices")
    } else if mask.bytes().all(|byte| byte == b'a') {
        base.to_string()
    } else {
        format!("{base}_mixed_{mask}")
    }
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
