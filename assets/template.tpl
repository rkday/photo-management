<!doctype html>

<html lang="en">
<head>
  <meta charset="utf-8">

  <title>The HTML5 Herald</title>
  <meta name="description" content="The HTML5 Herald">
  <meta name="author" content="SitePoint">

  <link rel="stylesheet" href="css/styles.css?v=1.0">

</head>

<body>

  <script>
  console.log("RKD");
  </script>
  <a href="/picture?pic={{prev}}" id='prevPage' accesskey="b">Back</a>
  <img src="http://localhost:8080/{{ image }}" height=600/>
  <a href="/picture?pic={{next}}" id='nextPage' accesskey="n">Next</a>
  <a href="/listdir?dir=./{{dir}}">Up</a>
  <form>
	  <input />
  </form>
  <script src="https://code.jquery.com/jquery-3.3.1.min.js"></script>
  <script>
  console.log("RKD");
$(window).keypress(function (e) {
  if (e.key === ' ' || e.key === 'Spacebar') {
    // ' ' is standard, 'Spacebar' was used by IE9 and Firefox < 37
    if (document.activeElement.nodeName != 'INPUT') {
    //e.preventDefault()
    console.log('Space pressed');
	  console.log(document.activeElement.nodeName);
    //$('#nextPage').trigger('click');
	  window.location = $('#nextPage').attr('href');
  }
	  }
})
  </script>
</body>
</html>
