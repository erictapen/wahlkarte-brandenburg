function loadJSON(path, success, error)
{
    var xhr = new XMLHttpRequest();
    xhr.onreadystatechange = function()
    {
        if (xhr.readyState === XMLHttpRequest.DONE) {
            if (xhr.status === 200) {
                if (success)
                    success(JSON.parse(xhr.responseText));
            } else {
                if (error)
                    error(xhr);
            }
        }
    };
    xhr.open("GET", path, true);
    xhr.send();
}

loadJSON('boundaries.json',
  function(data) { L.geoJSON(data).addTo(mymap); },
  function(xhr) { console.error(xhr); }
);

var mymap = L.map('mapid').setView([52.392, 13.387], 8);

