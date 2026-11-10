extern crate skeptic;
#[ignore]
#[test] fn async_impl_sect_async_example_line_22() {
    let s = &format!(r####"
{}"####, r####"extern crate ext_php_rs;
extern crate php_tokio;
extern crate reqwest;
use ext_php_rs::prelude::*;
use php_tokio::{php_async_impl, EventLoop};

#[php_class]
struct Client {}

#[php_async_impl]
impl Client {
    pub fn init() -> PhpResult<u64> {
        EventLoop::init()
    }
    pub fn wakeup() -> PhpResult<()> {
        EventLoop::wakeup()
    }
    pub async fn get(url: &str) -> anyhow::Result<String> {
        Ok(reqwest::get(url).await?.text().await?)
    }
}

pub extern "C" fn request_shutdown(_type: i32, _module_number: i32) -> i32 {
    EventLoop::shutdown();
    0
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.request_shutdown_function(request_shutdown)
}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn bailout_guard_sect_the_problem_line_17() {
    let s = &format!(r####"
{}"####, r####"#[php_function]
pub fn process_file(callback: ZendCallable) {
    let file = File::open("data.txt").unwrap();

    // A fatal error in the callback leaks the file handle
    callback.try_call(vec![]);
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn bailout_guard_sect_using_line_32() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use std::fs::File;

#[php_function]
pub fn process_file(callback: ZendCallable) {
    let file = BailoutGuard::new(File::open("data.txt").unwrap());

    // The file is closed even if a bailout occurs
    callback.try_call(vec![]);

    // Use the file via Deref
    // file.read_to_string(...);
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn bailout_guard_sect_using_line_71() {
    let s = &format!(r####"
{}"####, r####"// Create a guard
let guard = BailoutGuard::new(value);

// Access the value (implements Deref and DerefMut)
guard.do_something();
let inner: &T = &*guard;
let inner_mut: &mut T = &mut *guard;

// Explicitly get references
let inner: &T = guard.get();
let inner_mut: &mut T = guard.get_mut();

// Extract the value and release the cleanup entry
let value: T = guard.into_inner();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn bailout_guard_sect_nested_calls_line_106() {
    let s = &format!(r####"
{}"####, r####"#[php_function]
pub fn outer_function(callback: ZendCallable) {
    let _outer_resource = BailoutGuard::new(Resource::new());

    inner_function(&callback);
}

fn inner_function(callback: &ZendCallable) {
    let _inner_resource = BailoutGuard::new(Resource::new());

    // On bailout, the try_catch of outer_function drops both resources
    callback.try_call(vec![]);
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn bailout_guard_sect_catching_a_bailout_yourself_line_131() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::zend::try_catch;

let connection = BailoutGuard::new(Connection::open());

let result = try_catch(|| {
    let _tmp = BailoutGuard::new(TempFile::create());
    risky_engine_call();
});

// On Err, try_catch dropped _tmp. connection is still valid here.
connection.query("...");
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn custom_sapi_sect_defining_a_line_20() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::embed::{ServerContext, RequestInfo};

struct MyContext {
    method: String,
    uri: String,
    body: Vec<u8>,
    body_offset: usize,
    finished: bool,
}

impl ServerContext for MyContext {
    fn init_request_info(&self, info: &mut RequestInfo) {
        info.request_method = Some(self.method.clone());
        info.request_uri = Some(self.uri.clone());
        info.content_length = self.body.len() as i64;
    }

    fn read_post(&mut self, buf: &mut [u8]) -> usize {
        let remaining = &self.body[self.body_offset..];
        let n = buf.len().min(remaining.len());
        buf[..n].copy_from_slice(&remaining[..n]);
        self.body_offset += n;
        n
    }

    fn read_cookies(&self) -> Option<&str> {
        None
    }

    fn finish_request(&mut self) -> bool {
        if self.finished {
            return false;
        }
        self.finished = true;
        true
    }

    fn is_request_finished(&self) -> bool {
        self.finished
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn custom_sapi_sect_implementing_the_trait_line_66() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::embed::{Sapi, ServerContext, RequestInfo, ServerVarRegistrar};

struct MyContext { method: String, uri: String, body: Vec<u8>, body_offset: usize, finished: bool }
impl ServerContext for MyContext {
    fn init_request_info(&self, _: &mut RequestInfo) {}
    fn read_post(&mut self, _: &mut [u8]) -> usize { 0 }
    fn read_cookies(&self) -> Option<&str> { None }
    fn finish_request(&mut self) -> bool { true }
    fn is_request_finished(&self) -> bool { true }
}
struct MySapi;

impl Sapi for MySapi {
    type Context = MyContext;

    fn name() -> &'static str { "my-sapi" }
    fn pretty_name() -> &'static str { "My Custom SAPI" }

    fn ub_write(_ctx: &mut MyContext, buf: &[u8]) -> usize {
        // Forward output to your HTTP response
        print!("{}", String::from_utf8_lossy(buf));
        buf.len()
    }

    fn log_message(msg: &str, _syslog_type: i32) {
        eprintln!("[php] {msg}");
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn custom_sapi_sect_building_and_starting_the_sapi_line_99() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::embed::Sapi;
struct MySapi;
impl Sapi for MySapi {
    type Context = ();
    fn name() -> &'static str { "x" }
    fn pretty_name() -> &'static str { "x" }
    fn ub_write(_ctx: &mut (), _buf: &[u8]) -> usize { 0 }
    fn log_message(_: &str, _: i32) {}
}
impl ext_php_rs::embed::ServerContext for () {
    fn init_request_info(&self, _: &mut ext_php_rs::embed::RequestInfo) {}
    fn read_post(&mut self, _: &mut [u8]) -> usize { 0 }
    fn read_cookies(&self) -> Option<&str> { None }
    fn finish_request(&mut self) -> bool { true }
    fn is_request_finished(&self) -> bool { true }
}

let module = MySapi::build_module().expect("failed to build SAPI module");
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn custom_sapi_sect_registering_variables_line_137() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::embed::{Sapi, ServerVarRegistrar};

struct MySapi;
struct Ctx;
impl ext_php_rs::embed::ServerContext for Ctx {
    fn init_request_info(&self, _: &mut ext_php_rs::embed::RequestInfo) {}
    fn read_post(&mut self, _: &mut [u8]) -> usize { 0 }
    fn read_cookies(&self) -> Option<&str> { None }
    fn finish_request(&mut self) -> bool { true }
    fn is_request_finished(&self) -> bool { true }
}
impl Sapi for MySapi {
    type Context = Ctx;
    fn name() -> &'static str { "x" }
    fn pretty_name() -> &'static str { "x" }
    fn ub_write(_: &mut Ctx, b: &[u8]) -> usize { b.len() }
    fn log_message(_: &str, _: i32) {}
    fn register_server_variables(
        _ctx: &mut Ctx,
        registrar: &mut ServerVarRegistrar,
    ) {
        registrar.register("SERVER_SOFTWARE", "my-server/1.0");
        registrar.register("SERVER_PROTOCOL", "HTTP/1.1");
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn embedded_php_sect_basic_usage_line_51() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::php_eval;

// Both forms work:
const SETUP: &[u8] = include_bytes!("../php/setup.php");
// const SETUP: &str = include_str!("../php/setup.php");

unsafe extern "C" fn on_request_start(
    _type: i32,
    _module_number: i32,
) -> i32 {
    if let Err(e) = php_eval::execute(SETUP) {
        eprintln!("Failed to run embedded PHP setup: {:?}", e);
    }
    0
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.request_startup_function(on_request_start)
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn embedded_php_sect_error_handling_line_127() {
    let s = &format!(r####"
{}"####, r####"if let Err(e) = php_eval::execute(SETUP_CODE) {
    match e {
        PhpEvalError::MissingOpenTag => {
            eprintln!("embedded PHP missing <?php open tag");
        }
        PhpEvalError::CompilationFailed => {
            eprintln!("embedded PHP syntax error");
        }
        PhpEvalError::ExecutionFailed => {
            eprintln!("embedded PHP threw an exception");
        }
        PhpEvalError::Bailout => {
            eprintln!("embedded PHP fatal error");
        }
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn module_globals_sect_defining_globals_line_14() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::zend::{ModuleGlobal, ModuleGlobals};

#[derive(Default)]
struct MyGlobals {
    request_count: i64,
    max_depth: i32,
}

impl ModuleGlobal for MyGlobals {
    fn ginit(&mut self) {
        self.max_depth = 512;
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn module_globals_sect_defining_globals_line_36() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::zend::{ModuleGlobal, ModuleGlobals};
#[derive(Default)]
struct SimpleGlobals {
    counter: i64,
}

impl ModuleGlobal for SimpleGlobals {}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn module_globals_sect_registering_globals_line_50() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::zend::{ModuleGlobal, ModuleGlobals};

#[derive(Default)]
struct MyGlobals { request_count: i64, max_depth: i32 }
impl ModuleGlobal for MyGlobals {}

static MY_GLOBALS: ModuleGlobals<MyGlobals> = ModuleGlobals::new();

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module.globals(&MY_GLOBALS)
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn module_globals_sect_accessing_globals_line_72() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::zend::{ModuleGlobal, ModuleGlobals};

#[derive(Default)]
struct MyGlobals { request_count: i64, max_depth: i32 }
impl ModuleGlobal for MyGlobals {}
static MY_GLOBALS: ModuleGlobals<MyGlobals> = ModuleGlobals::new();

#[php_function]
pub fn get_request_count() -> i64 {
    MY_GLOBALS.get().request_count
}

#[php_function]
pub fn increment_request_count() {
    unsafe { MY_GLOBALS.get_mut() }.request_count += 1;
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn module_globals_sect_advanced_raw_pointer_access_line_101() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::zend::{ModuleGlobal, ModuleGlobals};
#[derive(Default)]
struct MyGlobals { request_count: i64 }
impl ModuleGlobal for MyGlobals {}
static MY_GLOBALS: ModuleGlobals<MyGlobals> = ModuleGlobals::new();
let ptr: *mut MyGlobals = MY_GLOBALS.as_ptr();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn module_globals_sect_cleanup_line_114() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::zend::ModuleGlobal;
#[derive(Default)]
struct MyGlobals { handle: Option<u64> }
impl ModuleGlobal for MyGlobals {
    fn gshutdown(&mut self) {
        self.handle.take();
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn observer_sect_function_call_observer_line_25() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;
use ext_php_rs::zend::ExecuteData;
use std::sync::atomic::{AtomicU64, Ordering};

struct CallCounter {
    count: AtomicU64,
}

impl CallCounter {
    fn new() -> Self {
        Self {
            count: AtomicU64::new(0),
        }
    }
}

impl FcallObserver for CallCounter {
    fn should_observe(&self, info: &FcallInfo) -> bool {
        !info.is_internal
    }

    fn begin(&self, _execute_data: &ExecuteData) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }

    fn end(&self, _execute_data: &ExecuteData, _retval: Option<&Zval>) {}
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.fcall_observer(CallCounter::new)
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn observer_sect_error_observer_line_83() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

struct ErrorTracker {
    fatal_count: AtomicU64,
    warning_count: AtomicU64,
}

impl ErrorTracker {
    fn new() -> Self {
        Self {
            fatal_count: AtomicU64::new(0),
            warning_count: AtomicU64::new(0),
        }
    }
}

impl ErrorObserver for ErrorTracker {
    fn should_observe(&self, error_type: ErrorType) -> bool {
        (ErrorType::Fatal | ErrorType::Warning).contains(error_type)
    }

    fn on_error(&self, error: &ErrorInfo) {
        if ErrorType::Fatal.contains(error.error_type) {
            self.fatal_count.fetch_add(1, Ordering::Relaxed);

            if let Some(trace) = error.backtrace() {
                for frame in trace {
                    eprintln!("  at {}:{}",
                        frame.file.as_deref().unwrap_or("<internal>"),
                        frame.line
                    );
                }
            }
        } else {
            self.warning_count.fetch_add(1, Ordering::Relaxed);
        }
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.error_observer(ErrorTracker::new)
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn observer_sect_error_observer_line_139() {
    let s = &format!(r####"
{}"####, r####"ErrorType::Error            // E_ERROR
ErrorType::Warning          // E_WARNING
ErrorType::Parse            // E_PARSE
ErrorType::Notice           // E_NOTICE
ErrorType::CoreError        // E_CORE_ERROR
ErrorType::CoreWarning      // E_CORE_WARNING
ErrorType::CompileError     // E_COMPILE_ERROR
ErrorType::CompileWarning   // E_COMPILE_WARNING
ErrorType::UserError        // E_USER_ERROR (deprecated since PHP 8.4)
ErrorType::UserWarning      // E_USER_WARNING
ErrorType::UserNotice       // E_USER_NOTICE
ErrorType::Strict           // E_STRICT (deprecated since PHP 8.4)
ErrorType::RecoverableError // E_RECOVERABLE_ERROR
ErrorType::Deprecated       // E_DEPRECATED
ErrorType::UserDeprecated   // E_USER_DEPRECATED

// Convenience groups, with the values of the PHP headers you build against
ErrorType::All   // E_ALL, contains Strict before PHP 8.4
ErrorType::Fatal // E_FATAL_ERRORS: Error | CoreError | CompileError | UserError | RecoverableError | Parse
ErrorType::Core  // E_CORE: CoreError | CoreWarning
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn observer_sect_error_observer_line_175() {
    let s = &format!(r####"
{}"####, r####"fn on_error(&self, error: &ErrorInfo) {
    if let Some(trace) = error.backtrace() {
        for frame in trace {
            println!("{}::{}() at {}:{}",
                frame.class.as_deref().unwrap_or(""),
                frame.function.as_deref().unwrap_or("<main>"),
                frame.file.as_deref().unwrap_or("<internal>"),
                frame.line
            );
        }
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn observer_sect_exception_observer_line_196() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use std::sync::atomic::{AtomicU64, Ordering};

struct ExceptionTracker {
    exception_count: AtomicU64,
}

impl ExceptionTracker {
    fn new() -> Self {
        Self {
            exception_count: AtomicU64::new(0),
        }
    }
}

impl ExceptionObserver for ExceptionTracker {
    fn on_exception(&self, exception: &ExceptionInfo) {
        self.exception_count.fetch_add(1, Ordering::Relaxed);
        eprintln!("[EXCEPTION] {}: {} at {}:{}",
            exception.class_name,
            exception.message.as_deref().unwrap_or("<no message>"),
            exception.file.as_deref().unwrap_or("<unknown>"),
            exception.line
        );
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.exception_observer(ExceptionTracker::new)
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn observer_sect_exception_observer_line_250() {
    let s = &format!(r####"
{}"####, r####"impl ExceptionObserver for MyObserver {
    fn on_exception(&self, exception: &ExceptionInfo) {
        eprintln!("[EXCEPTION] {}: {}",
            exception.class_name,
            exception.message.as_deref().unwrap_or("<no message>")
        );

        if let Some(trace) = exception.backtrace() {
            for frame in trace {
                eprintln!("  at {}::{}() in {}:{}",
                    frame.class.as_deref().unwrap_or(""),
                    frame.function.as_deref().unwrap_or("<main>"),
                    frame.file.as_deref().unwrap_or("<internal>"),
                    frame.line
                );
            }
        }
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn observer_sect_zend_extension_handler_line_291() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::ffi::zend_op_array;
use ext_php_rs::zend::ExecuteData;
use std::sync::atomic::{AtomicU64, Ordering};

struct StatementProfiler { count: AtomicU64 }

impl ZendExtensionHandler for StatementProfiler {
    fn on_statement(&self, _execute_data: &ExecuteData) {
        self.count.fetch_add(1, Ordering::Relaxed);
    }
    fn on_activate(&self) {
        self.count.store(0, Ordering::Relaxed);
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .zend_extension(|| StatementProfiler { count: AtomicU64::new(0) })
        .hook_statements()
        .finish()
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn observer_sect_using_all_observers_line_365() {
    let s = &format!(r####"
{}"####, r####"#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .fcall_observer(MyProfiler::new)
        .error_observer(MyErrorTracker::new)
        .exception_observer(MyExceptionTracker::new)
        .zend_extension(MyStatementProfiler::new)
        .hook_statements()
        .finish()
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn worker_mode_sect_api_line_17() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::embed::{
    worker_request_shutdown,
    worker_request_startup,
    worker_reset_superglobals,
};

// After processing a request:
worker_request_shutdown();

// Before the next request:
worker_request_startup().expect("startup failed");
worker_reset_superglobals();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn worker_mode_sect_zts_and_line_52() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::embed::PhpThreadGuard;

std::thread::spawn(|| {
    let _guard = PhpThreadGuard::new();
    // This thread can now run PHP requests.
    // TLS is cleaned up when _guard is dropped.
});
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn worker_mode_sect_run_code_at_the_start_of_each_request_line_76() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::zend::{SapiRequestInfo, set_header, set_response_code};

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.sapi_activate_function(|info: &SapiRequestInfo| {
        if info.request_uri() == Some("/old") {
            let _ = set_header("Location: /new");
            let _ = set_response_code(301);
        }
    })
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn exceptions_sect_throwing_exceptions_line_31() {
    let s = &format!(r####"
{}"####, r####"let globals = ExecutorGlobals::get();
if let Some(exception) = globals.exception() {
    // inspect it; it stays owned by the engine
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn exceptions_sect_throwing_exceptions_line_49() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use std::convert::TryInto;

// Trivial example - PHP represents all integers as `u64` on 64-bit systems
// so the `u32` would be converted back to `u64`, but that's okay for an example.
#[php_function]
pub fn something_fallible(n: u64) -> PhpResult<u32> {
    let n: u32 = n.try_into().map_err(|_| "Could not convert into u32")?;
    Ok(n)
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn exceptions_sect_panics_line_117() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::zend::{ExecuteData, run_handler};
use ext_php_rs::types::Zval;
use std::panic::AssertUnwindSafe;

extern "C" fn my_handler(ex: &mut ExecuteData, retval: &mut Zval) {
    run_handler(AssertUnwindSafe(|| {
        // This code can panic, bail out or throw.
    }));
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn cargo_php_sect_stubs_line_132() {
    let s = &format!(r####"
{}"####, r####"#[cfg(not(debug_assertions))]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn hello_world_sect_writing_our_extension_line_67() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_function]
pub fn hello_world(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(hello_world))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn ini_builder_sect_ini_builder_line_13() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::builder::{IniBuilder, SapiBuilder};

fn main() {
// Create a new IniBuilder instance.
let mut builder = IniBuilder::new();

// Append a single key/value pair to the INIT buffer with an unquoted value.
builder.unquoted("log_errors", "1");

// Append a single key/value pair to the INI buffer with a quoted value.
builder.quoted("default_mimetype", "text/html");

// Append INI line text as-is. A line break will be automatically appended.
builder.define("memory_limit=128MB");

// Prepend INI line text as-is. No line break insertion will occur.
builder.prepend("error_reporting=0\ndisplay_errors=1\n");

// Construct and start the SAPI.
let mut sapi = SapiBuilder::new("name", "pretty_name").build()
  .expect("should build SAPI");
unsafe { sapi_startup(&raw mut sapi) };

// Hand the INI entries to PHP before the engine starts.
sapi.ini_entries = builder.finish().as_ptr();
unsafe { php_module_startup(&raw mut sapi, get_module()) };
}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn ini_settings_sect_registering_ini_settings_line_10() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::{IniEntryDef, IniEntryDefs};
use ext_php_rs::flags::IniEntryPermission;

static INI_ENTRIES: IniEntryDefs<2> = IniEntryDefs::new([
    IniEntryDef::new(
        c"my_extension.display_emoji",
        c"yes",
        IniEntryPermission::All,
    ),
    IniEntryDef::end(),
]);

pub fn startup(ty: i32, mod_num: i32) -> i32 {
    IniEntryDef::register(INI_ENTRIES.as_slice(), mod_num);

    0
}

#[php_module]
#[php(startup = "startup")]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn ini_settings_sect_getting_ini_settings_line_44() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{
    prelude::*,
    zend::ExecutorGlobals,
};

pub fn startup(ty: i32, mod_num: i32) -> i32 {
    // Get all INI values
    let ini_values = ExecutorGlobals::get().ini_values(); // HashMap<String, Option<String>>
    let my_ini_value = ini_values.get("my_extension.display_emoji"); // Option<Option<String>>

    0
}

#[php_module]
#[php(startup = "startup")]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn ini_settings_sect_memory_line_77() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::{flags::IniEntryPermission, zend::{IniEntryDef, IniEntryDefs}};

static INI: IniEntryDefs<2> = IniEntryDefs::new([
    IniEntryDef::new(c"my_extension.display_emoji", c"1", IniEntryPermission::All),
    IniEntryDef::end(),
]);

pub fn startup(_ty: i32, mod_num: i32) -> i32 {
    IniEntryDef::register(INI.as_slice(), mod_num);
    0
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn classes_sect_example_line_106() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_class]
pub struct Human {
    name: String,
    age: i32,
    #[php(prop)]
    address: String,
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<Human>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn classes_sect_example_line_129() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{
    prelude::*,
    exception::PhpException,
    zend::ce
};

#[php_class]
#[php(name = "Redis\\Exception\\RedisException")]
#[php(extends(ce = ce::exception, stub = "\\Exception"))]
#[derive(Default)]
pub struct RedisException;

// Throw our newly created exception
#[php_function]
pub fn throw_exception() -> PhpResult<i32> {
    Err(PhpException::from_class::<RedisException>("Not good!".into()))
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<RedisException>()
        .function(wrap_function!(throw_exception))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn classes_sect_example_line_163() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_class]
#[derive(Default)]
pub struct Animal;

#[php_impl]
impl Animal {
    pub fn speak(&self) -> &'static str {
        "..."
    }
}

#[php_class]
#[php(extends(Animal))]
#[derive(Default)]
pub struct Dog;

#[php_impl]
impl Dog {
    pub fn speak(&self) -> &'static str {
        "Woof!"
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<Animal>()
        .class::<Dog>()
}
fn main() {}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn classes_sect_example_line_208() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

/// Trait for shared behavior
trait AnimalBehavior {
    fn speak(&self) -> &'static str {
        "..."
    }
}

#[php_class]
#[derive(Default)]
pub struct Animal;

impl AnimalBehavior for Animal {}

#[php_impl]
impl Animal {
    pub fn speak(&self) -> &'static str {
        AnimalBehavior::speak(self)
    }
}

#[php_class]
#[php(extends(Animal))]
#[derive(Default)]
pub struct Dog;

impl AnimalBehavior for Dog {
    fn speak(&self) -> &'static str {
        "Woof!"
    }
}

#[php_impl]
impl Dog {
    // Re-export the method so it works on Dog instances
    pub fn speak(&self) -> &'static str {
        AnimalBehavior::speak(self)
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .class::<Animal>()
        .class::<Dog>()
}
fn main() {}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn classes_sect_implementing_an_interface_line_271() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{
    prelude::*,
    exception::PhpResult,
    types::Zval,
    zend::ce,
};

#[php_class]
#[php(implements(ce = ce::arrayaccess, stub = "\\ArrayAccess"))]
#[derive(Default)]
pub struct EvenNumbersArray;

/// Returns `true` if the array offset is an even number.
/// Usage:
/// ```php
/// $arr = new EvenNumbersArray();
/// var_dump($arr[0]); // true
/// var_dump($arr[1]); // false
/// var_dump($arr[2]); // true
/// var_dump($arr[3]); // false
/// var_dump($arr[4]); // true
/// var_dump($arr[5] = true); // Fatal error:  Uncaught Exception: Setting values is not supported
/// ```
#[php_impl]
impl EvenNumbersArray {
    pub fn __construct() -> EvenNumbersArray {
        EvenNumbersArray {}
    }
    // We need to use `Zval` because ArrayAccess needs $offset to be a `mixed`
    pub fn offset_exists(&self, offset: &'_ Zval) -> bool {
        offset.is_long()
    }
    pub fn offset_get(&self, offset: &'_ Zval) -> PhpResult<bool> {
        let integer_offset = offset.long().ok_or("Expected integer offset")?;
        Ok(integer_offset % 2 == 0)
    }
    pub fn offset_set(&mut self, _offset: &'_ Zval, _value: &'_ Zval) -> PhpResult {
        Err("Setting values is not supported".into())
    }
    pub fn offset_unset(&mut self, _offset: &'_ Zval) -> PhpResult {
        Err("Setting values is not supported".into())
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<EvenNumbersArray>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn classes_sect_static_properties_line_333() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::class::RegisteredClass;

#[php_class]
pub struct Counter {
    #[php(prop)]
    pub instance_value: i32,
    #[php(prop, static, default = 0)]
    pub count: i32,
    #[php(prop, static, flags = ext_php_rs::flags::PropertyFlags::Private)]
    pub internal_state: String,
}

#[php_impl]
impl Counter {
    pub fn __construct(value: i32) -> Self {
        Self {
            instance_value: value,
            count: 0,
            internal_state: String::new(),
        }
    }

    /// Increment the static counter from Rust
    pub fn increment() {
        let ce = Self::get_metadata().ce();
        let current: i64 = ce.get_static_property("count").unwrap_or(0);
        ce.set_static_property("count", current + 1).unwrap();
    }

    /// Get the current count
    pub fn get_count() -> i64 {
        let ce = Self::get_metadata().ce();
        ce.get_static_property("count").unwrap_or(0)
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<Counter>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn classes_sect_abstract_classes_line_396() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::flags::ClassFlags;

#[php_class]
#[php(flags = ClassFlags::Abstract)]
pub struct AbstractAnimal;

#[php_impl]
impl AbstractAnimal {
    // Protected constructor for subclasses
    #[php(vis = "protected")]
    pub fn __construct() -> Self {
        Self
    }

    // Abstract method - must be implemented by subclasses.
    // Body is never called; use unimplemented!() as a placeholder.
    #[php(abstract)]
    pub fn speak(&self) -> String {
        unimplemented!()
    }

    // Concrete method - inherited by subclasses
    pub fn breathe(&self) {
        println!("Breathing...");
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn classes_sect_final_classes_line_455() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::flags::ClassFlags;

#[php_class]
#[php(flags = ClassFlags::Final)]
pub struct FinalClass;
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn classes_sect_readonly_classes_php_8_2_line_470() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_class]
#[php(readonly)]
pub struct ImmutablePoint {
    x: f64,
    y: f64,
}

#[php_impl]
impl ImmutablePoint {
    pub fn __construct(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn distance_from_origin(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<ImmutablePoint>()
}
fn main() {}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn classes_sect_readonly_classes_php_8_2_line_523() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::flags::ClassFlags;

// Readonly + Final class
#[php_class]
#[php(readonly)]
#[php(flags = ClassFlags::Final)]
pub struct FinalImmutableData {
    value: String,
}
fn main() {}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn classes_sect_readonly_classes_php_8_2_line_557() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs_build::{find_php, PHPInfo, ApiVersion, emit_php_cfg_flags, emit_check_cfg};

fn main() -> anyhow::Result<()> {
    let php = find_php()?;
    let info = PHPInfo::get(&php)?;
    let version: ApiVersion = info.zend_version()?.try_into()?;

    emit_check_cfg();
    emit_php_cfg_flags(version);
    Ok(())
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn classes_sect_readonly_classes_php_8_2_line_573() {
    let s = &format!(r####"
{}"####, r####"#[php_class]
#[cfg_attr(php82, php(readonly))]
pub struct MaybeReadonlyClass {
    value: String,
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn classes_sect_cloning_line_599() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;

#[php_class]
#[derive(Clone)]
pub struct Style {
    #[php(prop)]
    pub font_size: f64,
    #[php(prop)]
    pub color: String,
}

#[php_impl]
impl Style {
    pub fn __construct(font_size: f64, color: String) -> Self {
        Self { font_size, color }
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn classes_sect_implementing_iterator_line_643() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, zend::ce};

#[php_class]
#[php(implements(ce = ce::iterator, stub = "\\Iterator"))]
pub struct RangeIterator {
    start: i64,
    end: i64,
    current: i64,
    index: i64,
}

#[php_impl]
impl RangeIterator {
    /// Create a new range iterator from start to end (inclusive).
    pub fn __construct(start: i64, end: i64) -> Self {
        Self {
            start,
            end,
            current: start,
            index: 0,
        }
    }

    /// Return the current element.
    pub fn current(&self) -> i64 {
        self.current
    }

    /// Return the key of the current element.
    pub fn key(&self) -> i64 {
        self.index
    }

    /// Move forward to next element.
    pub fn next(&mut self) {
        self.current += 1;
        self.index += 1;
    }

    /// Rewind the Iterator to the first element.
    pub fn rewind(&mut self) {
        self.current = self.start;
        self.index = 0;
    }

    /// Checks if current position is valid.
    pub fn valid(&self) -> bool {
        self.current <= self.end
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<RangeIterator>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn classes_sect_implementing_iterator_line_734() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, zend::ce};

#[php_class]
#[php(implements(ce = ce::iterator, stub = "\\Iterator"))]
pub struct MapIterator {
    keys: Vec<String>,
    values: Vec<String>,
    index: usize,
}

#[php_impl]
impl MapIterator {
    pub fn __construct() -> Self {
        Self {
            keys: vec!["first".into(), "second".into(), "third".into()],
            values: vec!["one".into(), "two".into(), "three".into()],
            index: 0,
        }
    }

    pub fn current(&self) -> Option<String> {
        self.values.get(self.index).cloned()
    }

    pub fn key(&self) -> Option<String> {
        self.keys.get(self.index).cloned()
    }

    pub fn next(&mut self) {
        self.index += 1;
    }

    pub fn rewind(&mut self) {
        self.index = 0;
    }

    pub fn valid(&self) -> bool {
        self.index < self.keys.len()
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<MapIterator>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn constant_sect_examples_line_21() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_const]
const TEST_CONSTANT: i32 = 100;

#[php_const]
#[php(name = "I_AM_RENAMED")]
const TEST_CONSTANT_THE_SECOND: i32 = 42;

#[php_const]
const ANOTHER_STRING_CONST: &'static str = "Hello world!";

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .constant(wrap_constant!(TEST_CONSTANT))
        .constant(wrap_constant!(TEST_CONSTANT_THE_SECOND))
        .constant(("MANUAL_CONSTANT", ANOTHER_STRING_CONST, &[]))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn enum_sect_options_line_31() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_enum]
pub enum Suit {
    Hearts,
    Diamonds,
    Clubs,
    Spades,
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.enumeration::<Suit>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn enum_sect_backed_enums_line_61() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_enum]
pub enum Suit {
    #[php(value = "hearts")]
    Hearts,
    #[php(value = "diamonds")]
    Diamonds,
    #[php(value = "clubs")]
    Clubs,
    #[php(value = "spades")]
    Spades,
}
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.enumeration::<Suit>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn enum_sect_backed_enums_line_90() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_enum]
#[php(allow_native_discriminants)]
pub enum Suit {
    Hearts = 1,
    Diamonds = 2,
    Clubs = 3,
    Spades = 4,
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.enumeration::<Suit>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn extern_sect_example_line_39() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{
    prelude::*,
    types::Zval,
};

#[php_extern]
extern "C" {
    fn strpos(haystack: &str, needle: &str, offset: Option<i64>) -> Zval;
}

#[php_function]
pub fn my_strpos() {
    assert_eq!(unsafe { strpos("Hello", "e", None) }.long(), Some(1));
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(my_strpos))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn function_sect_optional_parameters_line_22() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_function]
pub fn greet(name: String, age: Option<i32>) -> String {
    let mut greeting = format!("Hello, {}!", name);

    if let Some(age) = age {
        greeting += &format!(" You are {} years old.", age);
    }

    greeting
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(greet))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn function_sect_optional_parameters_line_54() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_function]
#[php(defaults(offset = 0))]
pub fn rusty_strpos(haystack: &str, needle: &str, offset: i64) -> Option<usize> {
    let haystack: String = haystack.chars().skip(offset as usize).collect();
    haystack.find(needle)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(rusty_strpos))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn function_sect_optional_parameters_line_77() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

/// `age` will be deemed required and nullable rather than optional.
#[php_function]
pub fn greet(name: String, age: Option<i32>, description: String) -> String {
    let mut greeting = format!("Hello, {}!", name);

    if let Some(age) = age {
        greeting += &format!(" You are {} years old.", age);
    }

    greeting += &format!(" {}.", description);
    greeting
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(greet))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn function_sect_optional_parameters_line_110() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

/// `age` will be deemed required and nullable rather than optional,
/// while description will be optional.
#[php_function]
#[php(optional = "description")]
pub fn greet(name: String, age: Option<i32>, description: Option<String>) -> String {
    let mut greeting = format!("Hello, {}!", name);

    if let Some(age) = age {
        greeting += &format!(" You are {} years old.", age);
    }

    if let Some(description) = description {
        greeting += &format!(" {}.", description);
    }

    greeting
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(greet))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn function_sect_variadic_functions_line_149() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::Zval};

/// This can be called from PHP as `add(1, 2, 3, 4, 5)`
#[php_function]
pub fn add(number: u32, numbers:&[&Zval]) -> u32 {
    // numbers is a slice of 4 Zvals all of type long
    number
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(add))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn function_sect_performance_line_177() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
// Fast path: standalone functions
#[php_function]
pub fn fast(name: String, age: Option<i32>) -> String { name }

// Fast path: parameters with defaults
#[php_function]
#[php(defaults(offset = 0))]
pub fn also_fast(haystack: &str, offset: i64) -> i64 { offset }
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn function_sect_performance_line_192() {
    let s = &format!(r####"
{}"####, r####"#[php_impl]
impl MyClass {
    // Fast path: static methods
    pub fn create(n: i32) -> Self { /* ... */ }

    // Fast path: instance methods
    pub fn get_value(&self, offset: i32) -> i32 { /* ... */ }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn function_sect_performance_line_206() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::Zval};
// Slower path: variadic arguments require runtime parsing
#[php_function]
pub fn add(first: u32, rest: &[&Zval]) -> u32 { first }
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn impl_sect_methods_line_69() {
    let s = &format!(r####"
{}"####, r####"#[php_impl]
impl MyClass {
    // Static method - no self parameter
    pub fn create_default() -> Self {
        Self { /* ... */ }
    }

    // Instance method - takes &self
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn impl_sect_methods_line_96() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;

#[php_class]
pub struct SecureClass;

#[php_impl]
impl SecureClass {
    #[php(final)]
    pub fn secure_method(&self) -> &str {
        "This cannot be overridden"
    }

    // Final static methods are also supported
    #[php(final)]
    pub fn secure_static() -> i32 {
        42
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn impl_sect_methods_line_122() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::flags::ClassFlags;

#[php_class]
#[php(flags = ClassFlags::Abstract)]
pub struct AbstractShape;

#[php_impl]
impl AbstractShape {
    // Protected constructor for subclasses
    #[php(vis = "protected")]
    pub fn __construct() -> Self {
        Self
    }

    // Abstract method - subclasses must implement this.
    // The body is never called; use unimplemented!() as a placeholder.
    #[php(abstract)]
    pub fn area(&self) -> f64 {
        unimplemented!()
    }

    // Concrete method in abstract class
    pub fn describe(&self) -> String {
        format!("A shape with area {}", self.area())
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn impl_sect_constants_line_186() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_class]
pub struct Limits;

#[php_impl]
impl Limits {
    const MAX_USERS: i64 = 100;

    #[php(vis = "protected")]
    const MAX_RETRIES: i64 = 3;

    #[php(vis = "private", name = "SEED")]
    const RANDOM_SEED: i64 = 42;
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn impl_sect_example_line_252() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendClassObject};

#[php_class]
#[derive(Debug, Default)]
pub struct Human {
    name: String,
    age: i32,
    #[php(prop)]
    address: String,
}

#[php_impl]
impl Human {
    const MAX_AGE: i32 = 100;

    // No `#[constructor]` attribute required here - the name is `__construct`.
    pub fn __construct(name: String, age: i32) -> Self {
        Self {
            name,
            age,
            address: String::new()
        }
    }

    #[php(getter)]
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }

    #[php(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[php(getter)]
    pub fn get_age(&self) -> i32 {
        self.age
    }

    pub fn introduce(&self) {
        println!("My name is {} and I am {} years old. I live at {}.", self.name, self.age, self.address);
    }

    pub fn get_raw_obj(self_: &mut ZendClassObject<Human>) -> &mut ZendClassObject<Human> {
        dbg!(self_)
    }

    pub fn get_max_age() -> i32 {
        Self::MAX_AGE
    }
}
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<Human>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn interface_sect_example_line_39() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendClassObject};


#[php_interface]
#[php(name = "Rust\\TestInterface")]
trait Test {
    const TEST: &'static str = "TEST";

    fn co();

    #[php(defaults(value = 0))]
    fn set_value(&mut self, value: i32);
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .interface::<PhpInterfaceTest>()
}

fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn interface_sect_interface_inheritance_line_95() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::ce;

#[php_interface]
#[php(extends(ce = ce::throwable, stub = "\\Throwable"))]
#[php(name = "MyException")]
trait MyExceptionInterface {
    fn get_error_code(&self) -> i32;
}

fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn interface_sect_interface_inheritance_line_113() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_interface]
trait BaseInterface {
    fn base_method(&self) -> i32;
}

#[php_interface]
#[php(extends(BaseInterface))]
trait ExtendedInterface {
    fn extended_method(&self) -> String;
}

fn main() {}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn interface_sect_interface_inheritance_line_137() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_interface]
#[php(name = "Rust\\ParentInterface")]
trait ParentInterface {
    fn parent_method(&self) -> String;
}

// ChildInterface extends ParentInterface in PHP
#[php_interface]
#[php(name = "Rust\\ChildInterface")]
trait ChildInterface: ParentInterface {
    fn child_method(&self) -> String;
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .interface::<PhpInterfaceParentInterface>()
        .interface::<PhpInterfaceChildInterface>()
}

fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn interface_sect_example_line_185() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

// Define a custom interface
#[php_interface]
#[php(name = "Rust\\Greetable")]
trait Greetable {
    fn greet(&self) -> String;
}

// Define a class
#[php_class]
#[php(name = "Rust\\Greeter")]
pub struct Greeter {
    name: String,
}

#[php_impl]
impl Greeter {
    pub fn __construct(name: String) -> Self {
        Self { name }
    }

    // Note: No need to add greet() here - it's automatically
    // registered by #[php_impl_interface] below
}

// Implement the interface for the class
// This automatically registers greet() as a PHP method
#[php_impl_interface]
impl Greetable for Greeter {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .interface::<PhpInterfaceGreetable>()
        .class::<Greeter>()
}

fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn interface_sect_cross_crate_support_line_287() {
    let s = &format!(r####"
{}"####, r####"// my-interfaces/src/lib.rs
use ext_php_rs::prelude::*;

/// A serializable interface that can convert objects to JSON.
#[php_interface]
#[php(name = "MyInterfaces\\Serializable")]
pub trait Serializable {
    fn to_json(&self) -> String;
}

// Re-export the generated PHP interface struct for consumers
pub use PhpInterfaceSerializable;
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn interface_sect_cross_crate_support_line_320() {
    let s = &format!(r####"
{}"####, r####"// my-extension/src/lib.rs
use ext_php_rs::prelude::*;
use my_interfaces::Serializable;

#[php_class]
#[php(name = "MyExtension\\User")]
pub struct User {
    name: String,
    email: String,
}

#[php_impl]
impl User {
    pub fn __construct(name: String, email: String) -> Self {
        Self { name, email }
    }

    // Note: No need to add to_json() here - it's automatically
    // registered by #[php_impl_interface] below
}

// Register the interface implementation
// This automatically registers to_json() as a PHP method
#[php_impl_interface]
impl Serializable for User {
    fn to_json(&self) -> String {
        format!(r#"{{"name":"{}","email":"{}"}}"#, self.name, self.email)
    }
}

#[php_module]
pub fn module(module: ModuleBuilder) -> ModuleBuilder {
    module
        // Register the interface from the library crate
        .interface::<my_interfaces::PhpInterfaceSerializable>()
        .class::<User>()
}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn module_sect_usage_line_55() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{
    prelude::*,
    zend::ModuleEntry,
    info_table_start,
    info_table_row,
    info_table_end
};

#[php_const]
pub const MY_CUSTOM_CONST: &'static str = "Hello, world!";

#[php_class]
pub struct Test {
    a: i32,
    b: i32
}
#[php_function]
pub fn hello_world() -> &'static str {
    "Hello, world!"
}

/// Used by the `phpinfo()` function and when you run `php -i`.
/// This will probably be simplified with another macro eventually!
pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("my extension", "enabled");
    info_table_end!();
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .constant(wrap_constant!(MY_CUSTOM_CONST))
        .class::<Test>()
        .function(wrap_function!(hello_world))
        .info_function(php_module_info)
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn php_sect_attributes_line_9() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
#[php(name = "hi_world")]
#[php(defaults(a = 1, b = 2))]
fn hello_world(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn php_sect_attributes_line_22() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
#[php(name = "hi_world", defaults(a = 1, b = 2))]
fn hello_world(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn zval_convert_sect_structs_line_16() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[derive(ZvalConvert)]
pub struct ExampleClass<'a> {
    a: i32,
    b: String,
    c: &'a str
}

#[php_function]
pub fn take_object(obj: ExampleClass) {
    dbg!(obj.a, obj.b, obj.c);
}

#[php_function]
pub fn give_object() -> ExampleClass<'static> {
    ExampleClass {
        a: 5,
        b: "String".to_string(),
        c: "Borrowed",
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(take_object))
        .function(wrap_function!(give_object))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn zval_convert_sect_structs_line_67() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

// T must implement both `PartialEq<i32>` and `FromZval`.
#[derive(Debug, ZvalConvert)]
pub struct CompareVals<T: PartialEq<i32>> {
    a: T,
    b: T
}

#[php_function]
pub fn take_object(obj: CompareVals<i32>) {
    dbg!(obj);
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(take_object))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn zval_convert_sect_enums_line_113() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[derive(Debug, ZvalConvert)]
pub enum UnionExample<'a> {
    Long(u64), // Long
    ProperStr(&'a str), // Actual string - not a converted value
    ParsedStr(String), // Potentially parsed string, i.e. a double
    None // Zval did not contain anything that could be parsed above
}

#[php_function]
pub fn test_union(val: UnionExample) {
    dbg!(val);
}

#[php_function]
pub fn give_union() -> UnionExample<'static> {
    UnionExample::Long(5)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .function(wrap_function!(test_union))
        .function(wrap_function!(give_union))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_requires_an_unsafe_block_line_11() {
    let s = &format!(r####"
{}"####, r####"zval.set_resource(resource);
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_requires_an_unsafe_block_line_17() {
    let s = &format!(r####"
{}"####, r####"// SAFETY: `resource` is a valid Zend-managed resource whose owned reference is
// transferred into `zval`.
unsafe { zval.set_resource(resource) };
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_sapi_header_raw_constructors_removed_line_38() {
    let s = &format!(r####"
{}"####, r####"let header = SapiHeader::from_raw(raw_header);
let headers = SapiHeaders::from_raw(raw_headers);
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_sapi_header_raw_constructors_removed_line_47() {
    let s = &format!(r####"
{}"####, r####"fn send_header(_ctx: &mut MyContext, header: &SapiHeader) {
    if let Some((name, value)) = header.as_name_value() {
        // Send the header to the client.
    }
}

fn send_headers(
    _ctx: &mut MyContext,
    headers: &SapiHeaders,
) -> SendHeadersResult {
    let response_code = headers.http_response_code();
    SendHeadersResult::DoSend
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_void_return_type_for_functions_without_explicit_return_line_72() {
    let s = &format!(r####"
{}"####, r####"#[php_function]
pub fn do_something() {
    println!("Hello");
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_void_return_type_for_functions_without_explicit_return_line_89() {
    let s = &format!(r####"
{}"####, r####"// Before: worked but was incorrect
#[php_function]
pub fn get_value() {
    42  // implicitly returned, but no declared type
}

// After: must declare return type
#[php_function]
pub fn get_value() -> i32 {
    42
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_127() {
    let s = &format!(r####"
{}"####, r####"fn bump(ht: &ZendHashTable) {
    if let Some(zv) = ht.get_mut("counter") {
        zv.set_long(1);
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_137() {
    let s = &format!(r####"
{}"####, r####"fn bump(ht: &mut ZendHashTable) {
    if let Some(zv) = ht.get_mut("counter") {
        zv.set_long(1);
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_147() {
    let s = &format!(r####"
{}"####, r####"let first = ht.get_mut("test").unwrap();
let second = ht.get_mut("test").unwrap();  // error[E0499]
first.set_long(2);
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_163() {
    let s = &format!(r####"
{}"####, r####"let table = ExecutorGlobals::get().function_table_mut();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_169() {
    let s = &format!(r####"
{}"####, r####"let mut globals = ExecutorGlobals::get_mut();
let table = globals.function_table_mut();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_187() {
    let s = &format!(r####"
{}"####, r####"#[php_function]
pub fn count_items(mut iterable: Iterable) -> usize {
    iterable.iter().map_or(0, Iterator::count)
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_198() {
    let s = &format!(r####"
{}"####, r####"let iterable = Iterable::from_zval(zval);
let iterator = zval.traversable();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_205() {
    let s = &format!(r####"
{}"####, r####"let iterable = Iterable::from_zval_mut(zval);
let iterator = zval.traversable();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_223() {
    let s = &format!(r####"
{}"####, r####"let wrapper = StreamWrapper::get_mut("http").unwrap();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_229() {
    let s = &format!(r####"
{}"####, r####"// SAFETY: used within the current request, and no other reference to the
// `http` wrapper is live for the duration of this borrow.
let wrapper = unsafe { StreamWrapper::get_mut("http") }.unwrap();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_248() {
    let s = &format!(r####"
{}"####, r####"#[php_function]
pub fn normalise(ht: &ZendHashTable) {
    rescale(ht);
}

fn rescale(ht: &ZendHashTable) {
    for key in ["width", "height"] {
        if let Some(zv) = ht.get_mut(key) {
            if let Some(value) = zv.long() {
                zv.set_long(value * 2);
            }
        }
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_mutable_accessors_require_line_267() {
    let s = &format!(r####"
{}"####, r####"#[php_function]
pub fn normalise(ht: &mut ZendHashTable) {
    rescale(ht);
}

fn rescale(ht: &mut ZendHashTable) {
    for key in ["width", "height"] {
        if let Some(zv) = ht.get_mut(key) {
            if let Some(value) = zv.long() {
                zv.set_long(value * 2);
            }
        }
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_engine_input_accessors_return_instead_of_panicking_line_317() {
    let s = &format!(r####"
{}"####, r####"let get = ProcessGlobals::get().http_get_vars().to_owned();
let name = header.name();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_engine_input_accessors_return_instead_of_panicking_line_324() {
    let s = &format!(r####"
{}"####, r####"let get = ProcessGlobals::get().http_get_vars()?.to_owned();
let name = header.name()?;
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_types_moved_to_line_349() {
    let s = &format!(r####"
{}"####, r####"let raw = DataType::Long.as_u32();
let ty = DataType::from(raw);
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_types_moved_to_line_356() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;

let raw = DataType::Long.as_u32();
let ty = DataType::from_u32(raw);
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_types_moved_to_line_370() {
    let s = &format!(r####"
{}"####, r####"let is_interface = ClassFlags::from_bits(class.flags)
    .unwrap_or(ClassFlags::empty())
    .contains(ClassFlags::Interface);
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_types_moved_to_line_378() {
    let s = &format!(r####"
{}"####, r####"let is_interface = class.is_interface;
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_types_moved_to_line_394() {
    let s = &format!(r####"
{}"####, r####"let constant: Constant = ("MY_CONST".to_string(), docs).into();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_types_moved_to_line_400() {
    let s = &format!(r####"
{}"####, r####"let constant = Constant {
    name: "MY_CONST".into(),
    docs: docs.into(),
    value: abi::Option::None,
};
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_types_moved_to_line_439() {
    let s = &format!(r####"
{}"####, r####"const TYPE: DataType = DataType::Object(Some(MyClass::CLASS_NAME));
const TYPE: DataType = DataType::Object(None);

if let DataType::Object(Some(name)) = ty {
    println!("{name}");
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_types_moved_to_line_450() {
    let s = &format!(r####"
{}"####, r####"const TYPE: DataType = DataType::object(MyClass::CLASS_NAME);
const TYPE: DataType = DataType::ANY_OBJECT;

if let Some(name) = ty.class_name() {
    println!("{name}");
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_requires_an_unsafe_block_line_469() {
    let s = &format!(r####"
{}"####, r####"let values: &[u64] = u64::unpack_into(zend_str);
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_requires_an_unsafe_block_line_475() {
    let s = &format!(r####"
{}"####, r####"// Preferred: the checked entry point.
let values: &[u64] = zval.binary_slice::<u64>()?;

// Or, when the checks are done by hand:
// SAFETY: `zend_str.len` is a multiple of 8 and `val` is 8-byte aligned.
let values: &[u64] = unsafe { u64::unpack_into(zend_str) };
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_requires_an_unsafe_block_line_498() {
    let s = &format!(r####"
{}"####, r####"zval.set_ptr(ptr);
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_requires_an_unsafe_block_line_504() {
    let s = &format!(r####"
{}"####, r####"// SAFETY: `ptr` outlives the zval and the zval never reaches an engine table.
unsafe { zval.set_ptr(ptr) };
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_iterators_yield_owned_values_line_524() {
    let s = &format!(r####"
{}"####, r####"let values: Vec<&Zval> = iterator.iter()?.map(|(_, v)| v).collect();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_iterators_yield_owned_values_line_530() {
    let s = &format!(r####"
{}"####, r####"let values: Vec<Zval> = iterator.iter()?.map(|(_, v)| v).collect();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_returns_line_544() {
    let s = &format!(r####"
{}"####, r####"let obj: &'static mut ZendObject = zbox.into_raw();
zval.set_object(obj);
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_returns_line_551() {
    let s = &format!(r####"
{}"####, r####"let obj: *mut ZendObject = zbox.into_raw();
// SAFETY: `into_raw` yields a valid, exclusively owned object.
zval.set_object(unsafe { &mut *obj });
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_heap_leaking_constructors_removed_line_568() {
    let s = &format!(r####"
{}"####, r####"let sapi = builder.build()?.into_raw();
unsafe { sapi_startup(sapi) };
// ...
unsafe { cleanup_sapi_allocations(sapi); drop(Box::from_raw(sapi)) };
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_heap_leaking_constructors_removed_line_577() {
    let s = &format!(r####"
{}"####, r####"let mut sapi = builder.build()?;
unsafe { sapi_startup(&raw mut sapi) };
// ...
unsafe { cleanup_sapi_allocations(&raw mut sapi) };
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_removed_line_596() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::rc::PhpRc;

let count = obj.get_count();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_removed_line_604() {
    let s = &format!(r####"
{}"####, r####"let count = obj.ref_count();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_observer_merged_into_line_630() {
    let s = &format!(r####"
{}"####, r####"fn should_observe(&self, error_type: ErrorType) -> bool {
    (ErrorType::FATAL | ErrorType::USER_WARNING).contains(error_type)
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_observer_merged_into_line_638() {
    let s = &format!(r####"
{}"####, r####"fn should_observe(&self, error_type: ErrorType) -> bool {
    (ErrorType::Fatal | ErrorType::UserWarning).contains(error_type)
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_removed_line_686() {
    let s = &format!(r####"
{}"####, r####"unsafe { ext_php_rs::zend::cleanup_module_allocations(entry) };
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_registration_tables_are_owned_per_extension_line_710() {
    let s = &format!(r####"
{}"####, r####"// v0.15
let entry = builder.build()?;

// v0.16
let (entry, arg_info) = builder.build()?;
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_registration_tables_are_owned_per_extension_line_722() {
    let s = &format!(r####"
{}"####, r####"// v0.15
.registration(|ce| MY_META.set_ce(ce))

// v0.16
.registration(|ce, arg_info| {
    MY_META.set_ce(ce);
    MY_META.set_arg_info(arg_info);
})
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_registration_tables_are_owned_per_extension_line_739() {
    let s = &format!(r####"
{}"####, r####"// v0.15
let (entry, startup): (ModuleEntry, ModuleStartup) = builder.try_into()?;

// v0.16
let (entry, startup, owned): (ModuleEntry, ModuleStartup, ModuleAllocations) =
    builder.try_into()?;
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_registration_tables_are_owned_per_extension_line_757() {
    let s = &format!(r####"
{}"####, r####"let entries = vec![IniEntryDef::new(
    "my_ext.enabled".to_owned(),
    "1".to_owned(),
    &IniEntryPermission::All,
)];
IniEntryDef::register(entries, mod_num);
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_registration_tables_are_owned_per_extension_line_768() {
    let s = &format!(r####"
{}"####, r####"static INI: IniEntryDefs<2> = IniEntryDefs::new([
    IniEntryDef::new(c"my_ext.enabled", c"1", IniEntryPermission::All),
    IniEntryDef::end(),
]);

IniEntryDef::register(INI.as_slice(), mod_num);
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_the_error_model_line_822() {
    let s = &format!(r####"
{}"####, r####"// v0.15
match callable.try_call(vec![]) {
    Err(Error::Exception(obj)) => { /* the exception object */ }
    _ => {}
}

// v0.16
match callable.try_call(vec![]) {
    Err(Error::ExceptionPending { class }) => {
        let obj = ExecutorGlobals::take_exception();
    }
    _ => {}
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_the_error_model_line_874() {
    let s = &format!(r####"
{}"####, r####"// v0.15
Err(EmbedError::CatchError) => { /* was it a bailout? no way to tell */ }

// v0.16
Err(EmbedError::CatchError(CatchError::Bailout)) => { /* ... */ }
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_the_error_model_line_890() {
    let s = &format!(r####"
{}"####, r####"// v0.15
PhpException::default("something went wrong".into());

// v0.16
PhpException::from_message("something went wrong".into());
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_the_error_model_line_927() {
    let s = &format!(r####"
{}"####, r####"// v0.15
ht.remove("key");

// v0.16
let removed: Option<Zval> = ht.remove("key");
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_cleanup_belongs_to_the_frame_line_949() {
    let s = &format!(r####"
{}"####, r####"let file = BailoutGuard::new(File::open("data.txt")?);
let result = try_catch(|| {
    let _tmp = BailoutGuard::new(TempFile::create());
    risky_engine_call();
});
// On Err, _tmp is still allocated here. The exported handler frees both
// _tmp and file later, and file dangles.
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_cleanup_belongs_to_the_frame_line_961() {
    let s = &format!(r####"
{}"####, r####"let file = BailoutGuard::new(File::open("data.txt")?);
let result = try_catch(|| {
    let _tmp = BailoutGuard::new(TempFile::create());
    risky_engine_call();
});
// On Err, try_catch already dropped _tmp. file is still valid.
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_panics_stay_on_the_rust_side_of_the_ffi_boundary_line_989() {
    let s = &format!(r####"
{}"####, r####"// v0.15: the panic continued past this line
let result = try_catch(|| risky());

// v0.16
match try_catch(|| risky()) {
    Err(CatchError::Panic(message)) => std::panic::resume_unwind(Box::new(message)),
    Err(CatchError::Bailout) => { /* ... */ }
    Err(_) => { /* ... */ }
    Ok(value) => { /* ... */ }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_panics_stay_on_the_rust_side_of_the_ffi_boundary_line_1028() {
    let s = &format!(r####"
{}"####, r####"// v0.15
exception.throw()?;
exception.throw().expect("Failed to throw PHP exception.");

// v0.16
exception.throw();
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_class_constant_visibility_line_1130() {
    let s = &format!(r####"
{}"####, r####"builder.constant("MAX", 10, &[])?;
builder.dyn_constant("MAX", &10, &[])?;

fn constants() -> &'static [(&'static str, &'static dyn IntoZvalDyn, DocComments)] {
    &[("MAX", &10, &[])]
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_class_constant_visibility_line_1141() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::flags::ConstantFlags;

builder.constant("MAX", 10, &[], ConstantFlags::Public)?;
builder.dyn_constant("MAX", &10, &[], ConstantFlags::Public)?;

fn constants() -> &'static [(&'static str, &'static dyn IntoZvalDyn, DocComments, ConstantFlags)] {
    &[("MAX", &10, &[], ConstantFlags::Public)]
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn v0_16_sect_parameters_after_must_accept_a_missing_value_line_1250() {
    let s = &format!(r####"
{}"####, r####"// Before: compiled, `describe("Ann")` threw at run time.
#[php_function]
#[php(optional = age)]
pub fn describe(name: String, age: Option<i64>, city: String) -> String

// After: `city` has a default, or is `Option<String>`.
#[php_function]
#[php(optional = age, defaults(city = "\"Paris\""))]
pub fn describe(name: String, age: Option<i64>, city: String) -> String
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn output_sect_text_output_line_17() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;

#[php_function]
pub fn greet(name: &str) {
    php_print!("Hello, {}!", name);
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn output_sect_text_output_line_30() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;

#[php_function]
pub fn greet(name: &str) {
    php_println!("Hello, {}!", name);
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn output_sect_binary_safe_output_line_54() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;

#[php_function]
pub fn output_binary() -> i64 {
    // Write binary data with NUL bytes - will be captured by ob_start()
    let bytes_written = php_output!(b"Hello\x00World");
    bytes_written as i64
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn output_sect_binary_safe_output_line_71() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;

#[php_function]
pub fn output_binary() -> i64 {
    // Write a byte literal
    php_write!(b"Hello World").expect("write failed");

    // Write binary data with NUL bytes (would panic with php_print!)
    let bytes_written = php_write!(b"Hello\x00World").expect("write failed");

    // Write a byte slice
    let data: &[u8] = &[0x48, 0x65, 0x6c, 0x6c, 0x6f]; // "Hello"
    php_write!(data).expect("write failed");

    bytes_written as i64
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn output_sect_function_api_line_106() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::zend::output_write;

fn output_data(data: &[u8]) {
    let bytes_written = output_write(data);
    if bytes_written != data.len() {
        eprintln!("Warning: incomplete write");
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn output_sect_output_handlers_line_144() {
    let s = &format!(r####"
{}"####, r####"use std::borrow::Cow;

use ext_php_rs::prelude::*;
use ext_php_rs::zend::{OutputHandlerFlags, start_output_handler};

#[php_function]
pub fn start_uppercase() -> PhpResult<()> {
    start_output_handler("uppercase", 0, OutputHandlerFlags::Std, |input, _op| {
        Cow::Owned(input.to_ascii_uppercase())
    })?;
    Ok(())
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn output_sect_response_headers_line_201() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::zend::{set_header, set_response_code};

#[php_function]
pub fn moved(location: &str) -> PhpResult<()> {
    set_header(&format!("Location: {location}"))?;
    set_response_code(301)?;
    Ok(())
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn superglobals_sect_accessing_http_superglobals_line_34() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::ProcessGlobals;

#[php_function]
pub fn get_cookie(name: String) -> Option<String> {
    ProcessGlobals::get()
        .http_cookie_vars()?
        .get(name.as_str())
        .and_then(|zval| zval.string())
}

#[php_function]
pub fn get_query_param(name: String) -> Option<String> {
    ProcessGlobals::get()
        .http_get_vars()?
        .get(name.as_str())
        .and_then(|zval| zval.string())
}

#[php_function]
pub fn get_post_param(name: String) -> Option<String> {
    ProcessGlobals::get()
        .http_post_vars()?
        .get(name.as_str())
        .and_then(|zval| zval.string())
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn superglobals_sect_accessing_http_superglobals_line_71() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::ProcessGlobals;

#[php_function]
pub fn get_request_method() -> Option<String> {
    ProcessGlobals::get()
        .http_server_vars()?
        .get("REQUEST_METHOD")
        .and_then(|zval| zval.string())
}

#[php_function]
pub fn get_remote_addr() -> Option<String> {
    ProcessGlobals::get()
        .http_server_vars()?
        .get("REMOTE_ADDR")
        .and_then(|zval| zval.string())
}

#[php_function]
pub fn get_user_agent() -> Option<String> {
    ProcessGlobals::get()
        .http_server_vars()?
        .get("HTTP_USER_AGENT")
        .and_then(|zval| zval.string())
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn superglobals_sect_accessing_http_superglobals_line_105() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::ProcessGlobals;

#[php_function]
pub fn get_uploaded_file_name(field: String) -> Option<String> {
    let globals = ProcessGlobals::get();
    let files = globals.http_files_vars()?;

    // $_FILES structure: $_FILES['field']['name'], ['tmp_name'], ['size'], etc.
    files
        .get(field.as_str())?
        .array()?
        .get("name")
        .and_then(|zval| zval.string())
}

#[php_function]
pub fn get_uploaded_file_tmp_path(field: String) -> Option<String> {
    let globals = ProcessGlobals::get();
    let files = globals.http_files_vars()?;

    files
        .get(field.as_str())?
        .array()?
        .get("tmp_name")
        .and_then(|zval| zval.string())
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn superglobals_sect_accessing_http_superglobals_line_142() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::boxed::ZBox;
use ext_php_rs::types::ZendHashTable;
use ext_php_rs::zend::ProcessGlobals;

#[php_function]
pub fn get_all_cookies() -> Option<ZBox<ZendHashTable>> {
    Some(ProcessGlobals::get().http_cookie_vars()?.to_owned())
}

#[php_function]
pub fn get_all_get_params() -> Option<ZBox<ZendHashTable>> {
    Some(ProcessGlobals::get().http_get_vars()?.to_owned())
}

#[php_function]
pub fn get_server_vars() -> Option<ZBox<ZendHashTable>> {
    Some(ProcessGlobals::get().http_server_vars()?.to_owned())
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn superglobals_sect_sapi_request_information_line_171() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::SapiGlobals;

#[php_function]
pub fn get_request_info() -> Vec<String> {
    let globals = SapiGlobals::get();
    let request_info = globals.request_info();

    let mut info = Vec::new();

    if let Some(method) = request_info.request_method() {
        info.push(format!("Method: {}", method));
    }
    if let Some(uri) = request_info.request_uri() {
        info.push(format!("URI: {}", uri));
    }
    if let Some(query) = request_info.query_string() {
        info.push(format!("Query: {}", query));
    }
    if let Some(content_type) = request_info.content_type() {
        info.push(format!("Content-Type: {}", content_type));
    }
    info.push(format!("Content-Length: {}", request_info.content_length()));

    info
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn superglobals_sect_accessing_constants_line_221() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::ExecutorGlobals;

#[php_function]
pub fn get_php_version_constant() -> Option<String> {
    let globals = ExecutorGlobals::get();
    globals
        .constants()?
        .get("PHP_VERSION")
        .and_then(|zval| zval.string())
}

#[php_function]
pub fn constant_exists(name: String) -> bool {
    let globals = ExecutorGlobals::get();
    globals
        .constants()
        .is_some_and(|c| c.get(name.as_str()).is_some())
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn superglobals_sect_accessing_ini_values_line_248() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::zend::ExecutorGlobals;

#[php_function]
pub fn get_memory_limit() -> Option<String> {
    ExecutorGlobals::get()
        .ini_values()
        .get("memory_limit")
        .cloned()
        .flatten()
}

#[php_function]
pub fn get_all_ini_values() -> Vec<(String, String)> {
    ExecutorGlobals::get()
        .ini_values()
        .iter()
        .filter_map(|(k, v)| {
            v.as_ref().map(|val| (k.clone(), val.clone()))
        })
        .collect()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn superglobals_sect_thread_safety_line_287() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::zend::ExecutorGlobals;

fn example() {
    // Read-only access (most common)
    let globals = ExecutorGlobals::get();
    // ... use globals ...
    // Guard released when `globals` goes out of scope

    // Mutable access (rarely needed)
    let mut globals = ExecutorGlobals::get_mut();
    // ... modify globals ...
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn binary_sect_rust_usage_line_24() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::binary::Binary;

#[php_function]
pub fn test_binary(input: Binary<u32>) -> Binary<u32> {
    for i in input.iter() {
        println!("{}", i);
    }

    vec![5, 4, 3, 2, 1]
        .into_iter()
        .collect::<Binary<_>>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn binary_slice_sect_rust_usage_line_30() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::binary_slice::BinarySlice;

#[php_function]
pub fn test_binary_slice(input: BinarySlice<u8>) -> u8 {
    let mut sum = 0;
    for i in input.iter() {
        sum += i;
    }

    sum
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn bool_sect_rust_example_line_25() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn test_bool(input: bool) -> String {
    if input {
        "Yes!".into()
    } else {
        "No!".into()
    }
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn bool_sect_rust_example_taking_by_reference_line_51() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::types;
#[php_function]
pub fn test_bool(input: &mut types::Zval) {
    input.reference_mut().unwrap().set_bool(false);
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn class_object_sect_examples_line_15() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendClassObject};

#[php_class]
pub struct Example {
    foo: i32,
    bar: i32
}

#[php_impl]
impl Example {
    // ext-php-rs treats the method as associated due to the `self_` argument.
    // The argument _must_ be called `self_`.
    pub fn builder_pattern(
        self_: &mut ZendClassObject<Example>,
    ) -> &mut ZendClassObject<Example> {
        // do something with `self_`
        self_
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<Example>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn class_object_sect_examples_line_47() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

#[php_class]
pub struct Example {
    foo: i32,
    bar: i32
}

#[php_impl]
impl Example {
    pub fn make_new(foo: i32, bar: i32) -> Example {
        Example { foo, bar }
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.class::<Example>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn functions_sect_functions_methods_line_8() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;

use ext_php_rs::zend::Function;

#[php_function]
pub fn test_function() -> () {
    let var_dump = Function::try_from_function("var_dump").unwrap();
    let _ = var_dump.try_call(vec![&"abc"]);
}

#[php_function]
pub fn test_method() -> () {
    let f = Function::try_from_method("ClassName", "staticMethod").unwrap();
    let _ = f.try_call(vec![&"abc"]);
}

fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn hashmap_sect_rust_example_line_30() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use std::collections::HashMap;
#[php_function]
pub fn test_hashmap(hm: HashMap<String, String>) -> Vec<String> {
    for (k, v) in hm.iter() {
        println!("k: {} v: {}", k, v);
    }

    hm.into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn hashmap_sect_order_preserving_line_90() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use indexmap::IndexMap;

#[php_function]
pub fn test_indexmap(map: IndexMap<String, String>) -> IndexMap<String, i32> {
    // Order is preserved when iterating
    for (k, v) in map.iter() {
        println!("k: {} v: {}", k, v);
    }

    // Return an ordered map
    let mut result = IndexMap::new();
    result.insert("first".to_string(), 1);
    result.insert("second".to_string(), 2);
    result.insert("third".to_string(), 3);
    result  // Order preserved: first, second, third
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn hashmap_sect_and_line_145() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ArrayKey;
#[php_function]
pub fn test_vec_kv(vec: Vec<(String, String)>) -> Vec<String> {
    for (k, v) in vec.iter() {
        println!("k: {} v: {}", k, v);
    }

    vec.into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
}

#[php_function]
pub fn test_vec_arraykey(vec: Vec<(ArrayKey, String)>) -> Vec<String> {
    for (k, v) in vec.iter() {
        println!("k: {} v: {}", k, v);
    }

    vec.into_iter()
        .map(|(_, v)| v)
        .collect::<Vec<_>>()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn iterable_sect_rust_example_line_18() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::types::Iterable;
#[php_function]
pub fn test_iterable(mut iterable: Iterable) {
    for (k, v) in iterable.iter().expect("cannot rewind iterator") {
        println!("k: {} v: {}", k.string().unwrap(), v.string().unwrap());
    }
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn iterator_sect_rust_example_line_22() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendIterator;
#[php_function]
pub fn test_iterator(iterator: &mut ZendIterator) {
    for (k, v) in iterator.iter().expect("cannot rewind iterator") {
        // Note that the key can be anything, even an object
        // when iterating over Traversables!
        println!("k: {} v: {}", k.string().unwrap(), v.string().unwrap());
    }
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn numbers_sect_rust_example_line_25() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn test_numbers(a: i32, b: u32, c: f32) -> u8 {
    println!("a {} b {} c {}", a, b, c);
    0
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn object_sect_examples_line_20() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendObject};

// Take an object reference and also return it.
#[php_function]
pub fn take_obj(obj: &mut ZendObject) -> () {
    let _ = obj.try_call_method("hello", vec![&"arg1", &"arg2"]);
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(take_obj))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn object_sect_examples_line_40() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendObject};

// Take an object reference and also return it.
#[php_function]
pub fn take_obj(obj: &mut ZendObject) -> &mut ZendObject {
    let _ = obj.set_property("hello", 5);
    dbg!(obj)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(take_obj))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn object_sect_examples_line_61() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendObject, boxed::ZBox};

// Create a new `stdClass` and return it.
#[php_function]
pub fn make_object() -> ZBox<ZendObject> {
    let mut obj = ZendObject::new_stdclass();
    let _ = obj.set_property("hello", 5);
    obj
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.function(wrap_function!(make_object))
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn object_sect_lazy_objects_php_8_4_line_101() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendObject};

#[php_function]
pub fn check_lazy(obj: &ZendObject) -> String {
    if obj.is_lazy() {
        if obj.is_lazy_ghost() {
            if obj.is_lazy_initialized() {
                "Initialized lazy ghost".into()
            } else {
                "Uninitialized lazy ghost".into()
            }
        } else if obj.is_lazy_proxy() {
            if obj.is_lazy_initialized() {
                "Initialized lazy proxy".into()
            } else {
                "Uninitialized lazy proxy".into()
            }
        } else {
            "Unknown lazy type".into()
        }
    } else {
        "Not a lazy object".into()
    }
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn object_sect_lazy_objects_php_8_4_line_144() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, zend::ClassEntry};

#[php_function]
pub fn can_class_be_lazy(class_name: &str) -> bool {
    ClassEntry::try_find(class_name)
        .map(|ce| ce.can_be_lazy())
        .unwrap_or(false)
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn object_sect_lazy_objects_php_8_4_line_168() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::{prelude::*, types::ZendObject, boxed::ZBox};

#[php_function]
pub fn create_lazy_ghost(obj: &mut ZendObject) -> PhpResult<()> {
    let init_value = "initialized".to_string();
    obj.make_lazy_ghost(Box::new(move || {
        // Initialization logic - use captured state
        println!("Initializing with: {}", init_value);
    }) as Box<dyn Fn()>)?;
    Ok(())
}

#[php_function]
pub fn create_lazy_proxy(obj: &mut ZendObject) -> PhpResult<()> {
    obj.make_lazy_proxy(Box::new(|| {
        // Return the real instance
        Some(ZendObject::new_stdclass())
    }) as Box<dyn Fn() -> Option<ZBox<ZendObject>>>)?;
    Ok(())
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn option_sect_rust_example_line_26() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn test_option_null(input: Option<String>) -> Option<String> {
    input.map(|input| format!("Hello {}", input).into())
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn str_sect_rust_example_line_20() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn str_example(input: &str) -> String {
    format!("Hello {}", input)
}

#[php_function]
pub fn str_return_example() -> &'static str {
    "Hello from Rust"
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn string_sect_rust_example_line_19() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn str_example(input: String) -> String {
    format!("Hello {}", input)
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn vec_sect_rust_example_line_21() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
#[php_function]
pub fn test_vec(vec: Vec<String>) -> String {
    vec.join(" ")
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn vec_sect_rust_example_line_51() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use std::collections::HashSet;

#[php_function]
pub fn test_hashset(set: HashSet<String>) -> HashSet<String> {
    // Duplicates are automatically removed
    set
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn vec_sect_rust_example_line_87() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use std::collections::BTreeSet;

#[php_function]
pub fn test_btreeset(set: BTreeSet<String>) -> BTreeSet<String> {
    // Duplicates removed, values sorted
    set
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn vec_sect_rust_example_line_132() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use indexmap::IndexSet;

#[php_function]
pub fn test_indexset(set: IndexSet<String>) -> IndexSet<String> {
    // Order is preserved, duplicates removed
    for v in set.iter() {
        println!("v: {}", v);
    }
    set
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn zend_hashtable_sect_basic_operations_line_18() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, ZendStr};
use ext_php_rs::boxed::ZBox;

#[php_function]
pub fn create_array() -> ZBox<ZendHashTable> {
    let mut ht = ZendHashTable::new();

    // Push values (auto-incrementing numeric keys)
    ht.push("first").unwrap();
    ht.push("second").unwrap();

    // Insert with string keys
    ht.insert("name", "John").unwrap();
    ht.insert("age", 30i64).unwrap();

    // Insert at specific numeric index
    ht.insert_at_index(100, "at index 100").unwrap();
    
    // Insert with ZendStr keys
    // This allows bypassing repeated zend_string_init allocations and re-hashing
    // when working with pre-existing or interned PHP strings
    let key = ZendStr::new("hello", false);
    ht.insert(&key, "world");

    ht
}

#[php_function]
pub fn read_array(arr: &ZendHashTable) {
    // Get by string key
    if let Some(name) = arr.get("name") {
        println!("Name: {:?}", name.str());
    }

    // Get by numeric index
    if let Some(first) = arr.get_index(0) {
        println!("First: {:?}", first.str());
    }

    // Check length
    println!("Length: {}", arr.len());
    println!("Is empty: {}", arr.is_empty());

    // Iterate over key-value pairs
    for (key, value) in arr.iter() {
        println!("{}: {:?}", key, value);
    }
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn zend_hashtable_sect_entry_api_line_81() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use ext_php_rs::boxed::ZBox;

#[php_function]
pub fn entry_example() -> ZBox<ZendHashTable> {
    let mut ht = ZendHashTable::new();

    // Insert a default value if the key doesn't exist
    ht.entry("counter").or_insert(0i64).unwrap();

    // Modify the value if it exists, using and_modify
    ht.entry("counter")
        .and_modify(|v| {
            if let Some(n) = v.long() {
                v.set_long(n + 1);
            }
        })
        .or_insert(0i64)
        .unwrap();

    // Use or_insert_with for lazy initialization
    ht.entry("computed")
        .or_insert_with(|| "computed value")
        .unwrap();

    // Works with numeric keys too
    ht.entry(42i64).or_insert("value at index 42").unwrap();

    ht
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn zend_hashtable_sect_entry_api_line_125() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::types::{ZendHashTable, Entry, Zval};
use ext_php_rs::boxed::ZBox;

#[php_function]
pub fn match_entry() -> ZBox<ZendHashTable> {
    let mut ht = ZendHashTable::new();
    ht.insert("existing", "value").unwrap();

    // Pattern match on the entry
    match ht.entry("existing") {
        Entry::Occupied(entry) => {
            println!("Key {:?} exists with value {:?}",
                     entry.key(),
                     entry.get().and_then(Zval::str),
             );
        }
        Entry::Vacant(entry) => {
            println!("Key {:?} is vacant", entry.key());
            entry.insert("new value").unwrap();
        }
    }

    ht
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn zend_hashtable_sect_entry_api_line_160() {
    let s = &format!(r####"
{}"####, r####"#![cfg_attr(windows, feature(abi_vectorcall))]
extern crate ext_php_rs;
use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendHashTable;
use ext_php_rs::boxed::ZBox;

#[php_function]
pub fn count_words(words: Vec<String>) -> ZBox<ZendHashTable> {
    let mut counts = ZendHashTable::new();

    for word in words {
        counts.entry(word.as_str())
            .and_modify(|v| {
                if let Some(n) = v.long() {
                    v.set_long(n + 1);
                }
            })
            .or_insert(1i64)
            .unwrap();
    }

    counts
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn zend_hashtable_sect_entry_api_line_191() {
    let s = &format!(r####"
{}"####, r####"extern crate ext_php_rs;
use ext_php_rs::types::ZendHashTable;

fn expensive_computation(key: &str) -> String {
    format!("computed_{}", key)
}

fn get_or_compute(cache: &mut ZendHashTable, key: &str) -> String {
    let value = cache.entry(key)
        .or_insert_with_key(|k| expensive_computation(&k.to_string()))
        .unwrap();

    value.str().unwrap_or_default().to_string()
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[test] fn zend_hashtable_sect_entry_api_line_213() {
    let s = &format!(r####"
{}"####, r####"extern crate ext_php_rs;
use ext_php_rs::types::{ZendHashTable, Entry};

fn update_if_exists(ht: &mut ZendHashTable, key: &str, new_value: &str) -> bool {
    match ht.entry(key) {
        Entry::Occupied(mut entry) => {
            entry.insert(new_value).unwrap();
            true
        }
        Entry::Vacant(_) => false,
    }
}
fn main() {}
"####);
    skeptic::rt::compile_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn zval_ownership_sect_local_mutation_without_pass_by_reference_line_32() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::types::Separated;

#[php_function]
pub fn process_array(mut data: Separated) -> bool {
    let Some(ht) = data.array_mut() else {
        return false;
    };
    ht.push("appended").is_ok()
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

#[ignore]
#[test] fn zval_ownership_sect_modify_the_caller_s_variable_line_63() {
    let s = &format!(r####"
{}"####, r####"use ext_php_rs::prelude::*;
use ext_php_rs::types::PhpRef;

#[php_function]
pub fn increment(mut val: PhpRef) {
    if let Some(n) = val.long() {
        val.set_long(n + 1);
    }
}
"####);
    skeptic::rt::run_test(r#"/home/hoplite/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/ext-php-rs-0.16.4"#, r#"/tmp/hoplite/workspace/repo-repo_1453211695_gh_inst_180670734/rust/target/release/build/ext-php-rs-e9315c95c8eb8ae6/out"#, r#"x86_64-unknown-linux-gnu"#, s);
}

