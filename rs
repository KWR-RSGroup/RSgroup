<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>RS Group</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <header>
        <div class="logo">
            <h1>RS Group</h1>
        </div>
        <nav>
            <ul>
                <li><a href="#home">Home</a></li>
                <li><a href="#about">About</a></li>
                <li><a href="#services">Services</a></li>
                <li><a href="#contact">Contact</a></li>
            </ul>
        </nav>
    </header>

    <section id="hero">
        <div class="hero-content">
            <h2>Empowering Property Agents with Technology</h2>
            <p>Maximize your earnings with our competitive commission rates.</p>
            <a href="#contact" class="cta">Get Started</a>
        </div>
    </section>

    <section id="about">
        <h2>About RS Group</h2>
        <p>RS Group aims to offer property agents higher commission rates and greater transparency in listings. We connect developers with agents for a streamlined sales process.</p>
    </section>

    <section id="services">
        <h2>Our Services</h2>
        <ul>
            <li>Property Listings</li>
            <li>Agent Tools</li>
            <li>Developer Partnerships</li>
            <li>Market Analytics</li>
        </ul>
    </section>

    <footer>
        <p>&copy; 2024 RS Group. All rights reserved.</p>
    </footer>
</body>
</html>

body {
    font-family: Arial, sans-serif;
    margin: 0;
    padding: 0;
    box-sizing: border-box;
}

header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 20px;
    background-color: #333;
    color: white;
}

header .logo h1 {
    margin: 0;
}

nav ul {
    list-style: none;
    display: flex;
    margin: 0;
    padding: 0;
}

nav ul li {
    margin-left: 20px;
}

nav ul li a {
    color: white;
    text-decoration: none;
}

#hero {
    display: flex;
    justify-content: center;
    align-items: center;
    height: 100vh;
    background: url('hero-image.jpg') no-repeat center center/cover;
    color: white;
    text-align: center;
}

#hero .cta {
    display: inline-block;
    margin-top: 20px;
    padding: 10px 20px;
    background-color: #007bff;
    color: white;
    text-decoration: none;
    border-radius: 5px;
}

section {
    padding: 40px 20px;
}

section h2 {
    text-align: center;
    margin-bottom: 20px;
}

section p, section ul {
    text-align: center;
    margin: 0 auto;
    max-width: 800px;
}

footer {
    text-align: center;
    padding: 20px;
    background-color: #333;
    color: white;
}
