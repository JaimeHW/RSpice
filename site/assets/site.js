/* RSpice site — shared behaviors: theme toggle + code-copy buttons.
   The pre-paint theme bootstrap lives inline at the top of each page's <body>. */
(function () {
  "use strict";
  var body = document.body;

  function store(k, v) { try { localStorage.setItem(k, v); } catch (e) {} }

  document.querySelectorAll(".js-theme-flip").forEach(function (btn) {
    btn.addEventListener("click", function () {
      body.dataset.theme = body.dataset.theme === "dark" ? "light" : "dark";
      store("rspice-site-theme", body.dataset.theme);
    });
  });

  document.querySelectorAll(".code-copy").forEach(function (btn) {
    btn.addEventListener("click", function () {
      var fig = btn.closest(".code");
      var pre = fig ? fig.querySelector("pre") : null;
      var txt = pre ? pre.textContent : "";
      var done = function () {
        var prev = btn.textContent;
        btn.textContent = "Copied";
        setTimeout(function () { btn.textContent = prev; }, 1400);
      };
      if (navigator.clipboard && navigator.clipboard.writeText) {
        navigator.clipboard.writeText(txt).then(done, done);
      } else { done(); }
    });
  });
})();
