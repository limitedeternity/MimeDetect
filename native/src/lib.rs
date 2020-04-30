use neon::prelude::*;
use infer::Infer;
use std::path::Path;

mod mime_types;
use mime_types::create_mime_map;

struct BackgroundTask {
    path: String,
}

impl Task for BackgroundTask {
    type Output = String;
    type Error = ();
    type JsEvent = JsString;

    fn perform(&self) -> Result<String, ()> {
        let p = Path::new(&self.path);

        if !p.exists() {
            return Ok("ERR::FILE_NOT_FOUND".to_string());
        }

        if !p.is_file() {
            return Ok("ERR::NOT_A_FILE".to_string());
        }

        let info = Infer::new();
        let mime_map = create_mime_map();

        let res = match info.get_from_path(p) {
            Ok(opt) => match opt {
                Some(type_struct) => format!("{mime} (by magic)", mime=type_struct.mime),
                None => match p.extension().and_then(|s| s.to_str()).and_then(|s| mime_map.get(&s)) {
                    Some(mime) => format!("{mime} (by extension)", mime=mime),
                    None => "binary/octet-stream (unknown)".to_string(),
                },
            },
            Err(_) => "ERR::IO_ERROR".to_string(),
        };

        Ok(res)
    }

    fn complete(self, mut cx: TaskContext, result: Result<String, ()>) -> JsResult<JsString> {
        Ok(cx.string(result.unwrap()))
    }
}

fn run_task(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let p = cx.argument::<JsString>(0)?.value() as String;
    let cb = cx.argument::<JsFunction>(1)?;
    
    let task = BackgroundTask { path: p };
    task.schedule(cb);

    Ok(cx.undefined())
} 

register_module!(mut cx, {
    cx.export_function("getMimeType", run_task)?;
    Ok(())
});

