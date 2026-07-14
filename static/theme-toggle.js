(function () {
    var root = document.documentElement;
    var toggle = document.getElementById("theme-toggle");
    function effective() {
        var t = root.getAttribute("data-theme");
        if (t) return t;
        return "dark";
    }
    function updateLabel() {
        toggle.textContent = effective() === "dark" ? "[light mode]" : "[dark mode]";
    }
    updateLabel();
    toggle.addEventListener("click", function (e) {
        e.preventDefault();
        var next = effective() === "dark" ? "light" : "dark";
        root.setAttribute("data-theme", next);
        updateLabel();
    });
})();
