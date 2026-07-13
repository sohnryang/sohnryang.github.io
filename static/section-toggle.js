(function () {
    var toggles = document.querySelectorAll(".section-toggle");
    Array.prototype.forEach.call(toggles, function (toggle) {
        toggle.addEventListener("click", function (e) {
            e.preventDefault();
            var section = toggle.closest(".section");
            var expanded = section.classList.toggle("section--expanded");
            toggle.textContent = expanded ? "[- show less]" : "[+ show more]";
        });
    });
})();
