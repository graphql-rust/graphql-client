const js = import("./call_from_js");

js.then(js => {
  js.run();
});
