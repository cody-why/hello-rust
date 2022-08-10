/*** 
 * @Author: plucky
 * @Date: 2022-07-11 18:45:14
 * @LastEditTime: 2022-07-13 22:54:42
 * @Description: 
 */

use devise::ext::SpanDiagnosticExt;
use devise::{MetaItem, Spanned, Result, FromMeta};
use devise::proc_macro2;



/// 一个属性结构
pub struct Attribute {
    /// 标识
    pub status: Option<i32>,
    /// The function 
    pub function: syn::ItemFn,
}

/// We generate a full parser for the meta-item for great error messages.
#[derive(FromMeta)]
struct Meta {
    #[meta(naked)]
    code: Code,
}

/// `Some` if there's a code, `None` if it's `default`.
#[derive(Debug)]
struct Code(Option<i32>);

impl FromMeta for Code {
    fn from_meta(meta: &MetaItem) -> Result<Self> {

        if usize::from_meta(meta).is_ok() {
            let status = usize::from_meta(meta)?;
            Ok(Code(Some(status as i32)))
        } else if let MetaItem::Path(path) = meta {
            if path.is_ident("default") {
                Ok(Code(None))
            } else {
                Err(meta.span().error("expected `default`"))
            }
        } else {
            let msg = format!("expected integer or `default`, found {}", meta.description());
            Err(meta.span().error(msg))
        }
    }
}

impl Attribute {
    pub fn parse(args: proc_macro2::TokenStream, function: syn::ItemFn) -> Result<Self> {

        let attr: MetaItem = syn::parse2(quote::quote!(abc(#args)))?;
        let status = Meta::from_meta(&attr)
            .map(|meta| meta.code.0)
            .map_err(|diag| diag.help("`#[标识]` 只能是数字或'default"))?;
        
        Ok(Attribute { status, function })
    }
}
