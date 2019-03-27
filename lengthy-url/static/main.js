
function submitURL() {
    console.log("URL submitted");
    const url = document.getElementById("url_input").value;
    const data = {path: url};
    fetch("/new_url", {
      method: "POST",
      body: JSON.stringify(data),
       headers: {
                  "Content-Type": "application/json",
                  // "Content-Type": "application/x-www-form-urlencoded",
       },
    }).then(res => {
      res.json().then(function(result) {
        // TODO: Set result div visble, populate result href with url
        if (result["valid"]) {
            console.log ("Result: " + result["url"]);
        } else {
            console.log("invalid url");
        }
      });
    });
}
