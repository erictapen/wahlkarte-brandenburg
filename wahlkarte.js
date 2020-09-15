var mymap = L.map('mapid').setView([52.392, 13.387], 8)

var geojson = {}
var electionData
var partei

function highlightFeature(e) {
  var layer = e.target

  layer.setStyle({
    weight: 1.6,
  })

  if (!L.Browser.ie && !L.Browser.opera && !L.Browser.edge) {
    layer.bringToFront()
  }
  info.update(layer.feature.properties)
}

function resetHighlight(e) {
  var layer = e.target
  layer.setStyle({
    weight: 0.8,
  })
  info.update()
}

function zoomToFeature(e) {
  mymap.fitBounds(e.target.getBounds())
}

function onEachFeature(feature, layer) {
  layer.on({
    mouseover: highlightFeature,
    mouseout: resetHighlight,
    click: zoomToFeature,
  })
}

var info = L.control()

info.onAdd = function (map) {
  this._div = L.DomUtil.create('div', 'info') // create a div with a class "info"
  this.update()
  return this._div
}

// method that we will use to update the control based on feature properties passed
info.update = function (props) {
  if (!props) {
    this._div.innerHTML = 'Bewege den Cursor über ein Gebiet.'
  } else if (!electionData) {
    this._div.innerHTML = 'Warte darauf, dass Wahldaten geladen werden.'
  } else {
    absolute_votes = electionData['_absolute'][props.ags]
    relative_votes = electionData[partei][props.ags] / absolute_votes
    this._div.innerHTML = '<h4>Name: <b>' + props.name + '</b></h4>' +
      '<h4>Stimmen ' + partei + ': <b>' + (relative_votes * 100).toFixed(2) +
      '%</b></h4>' +
      '<h4>Gültige Stimmen insgesamt: <b>' + absolute_votes + '</b></h4>'
  }
}

function loadJSON(path, success, error) {
  var xhr = new XMLHttpRequest()
  xhr.onreadystatechange = function () {
    if (xhr.readyState === XMLHttpRequest.DONE) {
      if (xhr.status === 200) {
        if (success) {
          success(JSON.parse(xhr.responseText))
        }
      } else {
        if (error) {
          error(xhr)
        }
      }
    }
  }
  xhr.open('GET', path, true)
  xhr.send()
}

loadJSON('boundaries.json', function (data) {
  geojson = L.geoJSON(data, {
    onEachFeature: onEachFeature,
    style: {
      color: '#606060',
      weight: 0.8,
    },
  }).addTo(mymap)

  geojson.eachLayer(function (layer) {
    layer.setStyle({
      fillColor: '#ff0000',
    })
  })
  info.addTo(mymap)
  init()
}, function (xhr) {
  console.error(xhr)
})

// colors from https://colorbrewer2.org/#type=sequential&scheme=Greys&n=9
var colors = [
  '#000000',
  '#252525',
  '#525252',
  '#737373',
  '#969696',
  '#bdbdbd',
  '#d9d9d9',
  '#f0f0f0',
  '#ffffff'
]

// Reload (cached) election data, paint the individual regions
function updateMap(wahl, wahl_h, partei) {
    loadJSON('elections/' + wahl + '.json', function (data) {
      console.log(wahl + ' loaded')
      electionData = data
      geojson.eachLayer(function (layer) {
        ags = layer.feature.properties.ags
        fraction = Math.floor(
          (colors.length - 1) *
            ((electionData[partei][ags] / electionData['_absolute'][ags]) /
              electionData[partei]['_highest_ratio']),
        )
        layer.setStyle({
          fillColor: colors[fraction],
          fillOpacity: 1.0,
          weight: 0.8,
        })
      })
      info.update()
      document.title = partei + " - " + wahl_h + " - Wahlergebnisse Brandenburg"
    }, function (xhr) {
      console.error(xhr)
    })
}

function init() {
  console.log('Initializing...')

  parentElement = document.getElementById("menu")
  selectElement = document.createElement("select")
  selectElement.id = "select-wahl-partei"
  selectElement.style = "position: absolute; top: 10px; right: 10px;"
  parentElement.appendChild(selectElement)
  selectElement.addEventListener('change', event => {
    if (!event.target.value) {
      console.error("No value set?")
      return
    }
    console.log('start loading')
    wahl = JSON.parse(event.target.value)["wahl"]
    wahl_h = JSON.parse(event.target.value)["wahl_h"]
    partei = JSON.parse(event.target.value)["partei"]
    electionData = undefined // unset electionData, so we don't accidentally display data from former selection
    updateMap(wahl, wahl_h, partei)
  })
  loadJSON('elections/select_options.json', function (data) {
    for (var i = 0; i < data.length; i++) {
        wahl = data[i].name
        wahl_h = data[i]["human_readable_name"]
        data[i].parties.forEach(partei => {
          option = document.createElement("option")
          option.text = wahl_h + " - " + partei
          option.value = JSON.stringify( { wahl: wahl, wahl_h: wahl_h, partei: partei } )
          selectElement.appendChild(option)
        })
    }
  }, function (xhr) {
    console.error(xhr)
  })

  partei = "AfD"
  updateMap("eu2019", "Europawahl 2019", "AfD")
}
