(function () {
    var root = document.documentElement;
    var toggle = document.getElementById("theme-toggle");
    function effective() {
        var t = root.getAttribute("data-theme");
        if (t) return t;
        return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    }
    function updateLabel() {
        toggle.textContent = effective() === "dark" ? "[light mode]" : "[dark mode]";
    }
    updateLabel();
    toggle.addEventListener("click", function (e) {
        e.preventDefault();
        var next = effective() === "dark" ? "light" : "dark";
        root.setAttribute("data-theme", next);
        localStorage.setItem("theme", next);
        updateLabel();
    });
})();
