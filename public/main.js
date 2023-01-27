async function init() {
  let rustApp = null;

  try {
    rustApp = await import("../pkg");
  } catch (e) {
    console.error(e);
    return;
  }

  console.log(rustApp);

  const input = document.getElementById("upload");
  const fileReader = new FileReader();

  // removing extra string don't needed
  fileReader.onloadend = () => {
    const base64 = fileReader.result.replace(
      /^data:image\/(png|jpeg|jpg);base64,/,
      ""
    );
    let img_data_url = rustApp.grayscale(base64);
    document.getElementById("new-img").setAttribute("src", img_data_url);
  };

  //get image as string
  input.addEventListener("change", () => {
    fileReader.readAsDataURL(input.files[0]);
  });
}

init();
