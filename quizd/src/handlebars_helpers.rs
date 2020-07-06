use handlebars::{Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext};

#[derive(Clone, Copy)]
pub struct ObjPrinter;

impl HelperDef for ObjPrinter {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper,
        _: &Handlebars,
        _: &Context,
        _: &mut RenderContext,
        out: &mut dyn Output,
    ) -> HelperResult {
        let param = h.param(0).unwrap();

        out.write(&format!("<pre>{:#?}</pre>", param.value()))?;
        Ok(())
    }
}
