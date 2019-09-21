include_repo! (board)
include_repo! (page)

default_namespace! {
  page: {
    board: {},
  },
}

pub const fn main_controller() {
  return render!(
    <html>
      <head>
        <page::head />
      </head>
      <body>
        <page::body />
      </body>
    </html>
  )
}


create_ui_component! (name = MainController, { 
  @property let mut initial_counter: u32 = 0;
  @property let mut max_counter: u32 = 0;

  @state let counter: u32 = 0 {
    print!("Counter state changed uuh", self.counter)
  };

  @capsule let global_counter: CounterCapsule {
    print!("Global Counter state changed uuh", self.global_counter)
    if (self.max_counter <= self.global_counter) {
      self.counter = self.global_counter;
    }
  };

  HtmlRoot {
    Head {
      Page::Head(position: ::META)
    }
    Body {
      Page::Body(self.counter)
      Page::Body(self.counter + 1)
    }
  } 
})


// into -->
struct MainControllerCache {
  el_01: HtmlRoot,
  el_02: Head,
  el_03: Page::Head,
  el_04: Body,
  el_05: Page::Body,
  el_06: Page::Body,
}

struct MainController {
  pub initial_counter: u32;
  pub max_counter: u32;
  pub counter: u32;
  pub global_counter: Option<u32>;
}

impl_task! (name = RenderMainController, type = MainController, {
  // ...
});

impl Component<MainController> for MainController {
  fn create(initial_counter: Option<u32>, max_counter: Option<u32>) -> MainController {
    MainController {
      initial_counter = initial_counter.unwrap_or(0),
      max_counter = max_counter.unwrap_or(0),
      counter: 0,
      global_counter: None,
    }
  }

  fn render(&self) {
  }
}