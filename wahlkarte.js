var mymap = L.map('mapid').setView([52.392, 13.387], 8)

var geojson = {}

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
  this._div.innerHTML = '<h4>Name: </h4>' +
    (props ? '<b>' + props.name + '</b>' : 'Hover over a state') + '<br>'
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

// colors from https://colorbrewer2.org/#type=diverging&scheme=BrBG&n=11
var colors = [
  '#543005',
  '#8c510a',
  '#bf812d',
  '#dfc27d',
  '#f6e8c3',
  '#f5f5f5',
  '#c7eae5',
  '#80cdc1',
  '#35978f',
  '#01665e',
  '#003c30',
]

function init() {
  console.log('Initializing...')
  document.querySelector('.select-wahl').addEventListener('change', event => {
    if (!event.target.value) {
      return
    }
    console.log('start loading')
    loadJSON('elections/' + event.target.value + '.json', function (data) {
      console.log(event.target.value + ' loaded')
      electionData = data
    }, function (xhr) {
      console.error(xhr)
    })
  })
  document.querySelector('.select-partei').addEventListener('change', event => {
    if (!event.target.value) {
      return
    }
    console.log('Show party data of ' + event.target.value)
    party = event.target.value
    geojson.eachLayer(function (layer) {
      fraction = Math.floor(
        (colors.length - 1) *
          (electionData[party][layer.feature.properties.ags] / 1000),
      )
      layer.setStyle({
        fillColor: colors[fraction],
        fillOpacity: 1.0,
        weight: 0.8,
      })
    })
  })
}
