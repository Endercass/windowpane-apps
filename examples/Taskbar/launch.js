export default (contentElement, fs, name) => {
  let winroot = contentElement.parentElement;
  winroot.style.border = "none";
  winroot.style.bottom = 0;
  winroot.style.width = "100vw";
  winroot.style.height = "48px";
  winroot.style.borderRadius = "24px";
  winroot.style.borderBottomLeftRadius = 0;
  winroot.style.borderBottomRightRadius = 0;
  console.log(contentElement, winroot);
  var frame = document.createElement("iframe");
  frame.style.width = "100%";
  frame.style.height = "100%";
  frame.style.display = "absolute";
  contentElement.appendChild(frame);

  fs.readFile(`/app/${name}/bin/taskbar.html`, (err, data) => {
    err && console.log(err);
    frame.src = URL.createObjectURL(new Blob([data]));
  });
};
