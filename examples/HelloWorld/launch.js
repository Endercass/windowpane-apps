export default (contentElement, fs, name) => {
  var frame = document.createElement("iframe");
  frame.style.width = "100%";
  frame.style.height = "100%";
  frame.style.display = "absolute";
  contentElement.appendChild(frame);

  fs.readFile(`/app/${name}/bin/index.html`, (err, data) => {
    err && console.log(err);
    frame.src = URL.createObjectURL(new Blob([data]));
  });
};
