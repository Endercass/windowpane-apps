export default (contentElement, fs) => {
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

  // let req = window.indexedDB.open("glassfs", 1);
  // req.onsuccess = (evt) => {
  //   console.log("Success creating/accessing IndexedDB database");
  //   let db = req.result;

  //   db.onerror = function (event) {
  //     console.log("Error creating/accessing IndexedDB database");
  //   };

  //   console.log(db);

  //   var transaction = db.transaction(["files"], IDBTransaction.READ);

  //   transaction.objectStore("files").get("test2").onsuccess = (evt) => {
  //     let file = evt.target.result;
  //     frame.src = URL.createObjectURL(file);
  //   };
  // };

  fs.readFile("/test2", (err, data) => {
    err && console.log(err);
    frame.src = URL.createObjectURL(new Blob([data]));
  });
};
