pub const microblog_html:&str = r#"
<!DOCTYPE html>
<html lang="en">
<head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>bytebrowser</title>
        <style>
body {
        margin: 0;
        padding: 0;
        display: flex;
        flex-direction: column;
        justify-content: flex-start;
        background-color: #2c2c2c;
        color:white;
        height: 100vh;
}

header {
        background-color: #111;
        color: white;
        padding: 10px;
        letter-spacing: 1px;
	justify-content: center; /* Horizontally center */
}

header h1 {
        margin: 0;
        font-size:1.5em;
}

h2, h3{
        font-weight: normal;
        margin-bottom: 10px;
        font-size: 1em;
	display: inline;
	color: #9ad2d8;
}

p {
        color: white;
}

.container {
        padding: 15px;
        gap: 15px;
}

a {
        color: #9ad2d8;
        text-decoration: none;
}
a:hover {
        text-decoration: underline;
}

a:visited {
        color: #ffaa29;
}

.columns {
	justify-content: center; /* Horizontally center */
	padding: 10px;
        display: flex;
        flex: 1;
        overflow: hidden;
        height:80%;
}

.left-column {
	font-family: monospace;
	width: auto;
        overflow-x: auto;
	overflow-y: auto;
	white-space: nowrap;
	outline: 1px solid white;
	padding-left: 10px;
	padding-right: 10px;
}

</style>
</head>
<body>
    <center>{RIGHT_MENU}</center>
    <div class="columns">
		<div class="left-column">
		{CONTENT}
		</div>
    </div>

</body>
</html>
"#;
