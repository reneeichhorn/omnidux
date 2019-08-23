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