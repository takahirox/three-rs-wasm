use web_time::Instant;

/// A monotonic clock; browser builds use Performance.now through web-time.
pub struct Clock {
    pub auto_start: bool,
    pub running: bool,
    pub elapsed_time: f64,
    origin: Instant,
    start_time: f64,
    old_time: f64,
}
impl Default for Clock {
    fn default() -> Self {
        Self {
            auto_start: true,
            running: false,
            elapsed_time: 0.0,
            origin: Instant::now(),
            start_time: 0.0,
            old_time: 0.0,
        }
    }
}
impl Clock {
    pub fn start(&mut self) {
        self.start_at(self.origin.elapsed().as_secs_f64());
    }
    pub fn start_at(&mut self, seconds: f64) {
        self.start_time = seconds;
        self.old_time = seconds;
        self.elapsed_time = 0.0;
        self.running = true;
    }
    pub fn start_time(&self) -> f64 {
        self.start_time
    }
    pub fn old_time(&self) -> f64 {
        self.old_time
    }
    pub fn get_delta(&mut self) -> f64 {
        self.delta_at(self.origin.elapsed().as_secs_f64())
    }
    pub fn delta_at(&mut self, seconds: f64) -> f64 {
        if self.auto_start && !self.running {
            self.start_at(seconds);
            return 0.0;
        }
        if !self.running {
            return 0.0;
        }
        let delta = seconds - self.old_time;
        self.old_time = seconds;
        self.elapsed_time += delta;
        delta
    }
    pub fn get_elapsed_time(&mut self) -> f64 {
        self.get_delta();
        self.elapsed_time
    }
    pub fn stop(&mut self) {
        self.get_elapsed_time();
        self.running = false;
        self.auto_start = false;
    }
}

pub struct Timer {
    origin: Instant,
    current: f64,
    delta: f64,
    elapsed: f64,
    timescale: f64,
    #[cfg(target_arch = "wasm32")]
    visibility: Option<VisibilityConnection>,
}
impl Default for Timer {
    fn default() -> Self {
        Self {
            origin: Instant::now(),
            current: 0.0,
            delta: 0.0,
            elapsed: 0.0,
            timescale: 1.0,
            #[cfg(target_arch = "wasm32")]
            visibility: None,
        }
    }
}
impl Timer {
    pub fn update(&mut self) {
        self.update_at(self.origin.elapsed().as_secs_f64());
    }
    /// Seconds since this timer's origin. Explicit timestamps make simulation tests deterministic.
    pub fn update_at(&mut self, seconds: f64) {
        #[cfg(target_arch = "wasm32")]
        if let Some(connection) = &self.visibility {
            if connection.document.hidden() {
                self.delta = 0.0;
                return;
            }
            if let Some(reset) = connection.reset.take() {
                self.current = reset;
            }
        }
        self.delta = (seconds - self.current) * self.timescale;
        self.current = seconds;
        self.elapsed += self.delta;
    }
    pub fn get_delta(&self) -> f64 {
        self.delta
    }
    pub fn get_elapsed(&self) -> f64 {
        self.elapsed
    }
    pub fn get_timescale(&self) -> f64 {
        self.timescale
    }
    pub fn set_timescale(&mut self, timescale: f64) {
        self.timescale = timescale;
    }
    pub fn reset(&mut self) {
        self.current = self.origin.elapsed().as_secs_f64();
    }
    #[cfg(target_arch = "wasm32")]
    pub fn connect(&mut self, document: web_sys::Document) -> crate::Result<()> {
        use wasm_bindgen::JsCast;
        self.disconnect();
        let reset = std::rc::Rc::new(std::cell::Cell::new(None));
        let value = reset.clone();
        let doc = document.clone();
        let origin = self.origin;
        let callback =
            wasm_bindgen::closure::Closure::wrap(Box::new(move |_event: web_sys::Event| {
                if !doc.hidden() {
                    value.set(Some(origin.elapsed().as_secs_f64()));
                }
            }) as Box<dyn FnMut(web_sys::Event)>);
        document
            .add_event_listener_with_callback("visibilitychange", callback.as_ref().unchecked_ref())
            .map_err(|e| crate::Error::Asset(format!("{e:?}")))?;
        self.visibility = Some(VisibilityConnection {
            document,
            reset,
            callback,
        });
        Ok(())
    }
    #[cfg(target_arch = "wasm32")]
    pub fn disconnect(&mut self) {
        self.visibility = None;
    }
    pub fn dispose(self) {}
}

#[cfg(target_arch = "wasm32")]
struct VisibilityConnection {
    document: web_sys::Document,
    reset: std::rc::Rc<std::cell::Cell<Option<f64>>>,
    callback: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::Event)>,
}
#[cfg(target_arch = "wasm32")]
impl Drop for VisibilityConnection {
    fn drop(&mut self) {
        use wasm_bindgen::JsCast;
        let _ = self.document.remove_event_listener_with_callback(
            "visibilitychange",
            self.callback.as_ref().unchecked_ref(),
        );
    }
}
