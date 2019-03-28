
function submitURL() {
    let good = document.querySelector('#success_result');
    let bad = document.querySelector('#failure_result');
    good.style.visibility = "hidden";
    bad.style.visibility = "hidden";
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
        if (result["valid"]) {
            console.log ("Result: " + result["url"]);
            good.style.visibility = "visible";
            bad.style.visibility = "hidden";
            let longUrl = document.getElementById('long_url');
            longUrl.href = "/tm/" + result["url"];
            longUrl.textContent = "http://LengthyURL.com/tm/" + result["url"];
        } else {
            console.log("invalid url");
            good.style.visibility = "hidden";
            bad.style.visibility = "visible";
        }
      });
    });
}
