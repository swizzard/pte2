const cl = "clicked";
Array.from(document.getElementsByClassName("repButton")).forEach(($el) => {
  // $el.onclick = (evt) => console.log(JSON.stringify(evt));
  $el.onclick = function () {
    this.classList.contains(cl)
      ? this.classList.remove(cl)
      : this.classList.add(cl);
  };
});
Array.from(document.getElementsByClassName("resetButton")).forEach(($el) => {
  $el.onclick = function () {
    Array.from(this.parentElement.children).forEach(($el) => {
      $el.classList.remove(cl);
    });
  };
});
document.getElementById("resetAll").onclick = function () {
  Array.from(document.getElementsByTagName("button")).forEach(($el) => {
    $el.classList.remove(cl);
  });
};
