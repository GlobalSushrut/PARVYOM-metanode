<!doctype html>
<!--
  Minimal Mistakes Jekyll Theme 4.26.1 by Michael Rose
  Copyright 2013-2024 Michael Rose - mademistakes.com | @mmistakes
  Free for personal and commercial use under the MIT license
  https://github.com/mmistakes/minimal-mistakes/blob/master/LICENSE
-->

<html lang="en" class="no-js">
  <head>
    <meta charset="utf-8">

<!-- begin _includes/seo.html --><title>News - Remmina</title>
<meta name="description" content="Remmina is a free, open-source tool for handling all your remote connection needs for protocols like RDP, SSH, SPICE, and VNC">


  <meta name="author" content="Remmina Project">
  


<meta property="og:type" content="website">
<meta property="og:locale" content="en_US">
<meta property="og:site_name" content="Remmina">
<meta property="og:title" content="News">
<meta property="og:url" content="https://remmina.org/news/remmina_news.php">


  <meta property="og:description" content="Remmina is a free, open-source tool for handling all your remote connection needs for protocols like RDP, SSH, SPICE, and VNC">





  <meta name="twitter:site" content="@RemminaProject">
  <meta name="twitter:title" content="News">
  <meta name="twitter:description" content="Remmina is a free, open-source tool for handling all your remote connection needs for protocols like RDP, SSH, SPICE, and VNC">
  <meta name="twitter:url" content="https://remmina.org/news/remmina_news.php">

  
    <meta name="twitter:card" content="summary">
    
  

  
    <meta name="twitter:creator" content="@RemminaProject">
  







  

  


<link rel="canonical" href="https://remmina.org/news/remmina_news.php">












<!-- end _includes/seo.html -->



  <link href="/feed.xml" type="application/atom+xml" rel="alternate" title="Remmina Feed">
<meta name="viewport" content="width=device-width, initial-scale=1.0">

<script type="text/javascript">
  document.documentElement.className = document.documentElement.className.replace(/\bno-js\b/g, '') + ' js ';
  
</script>

<!-- For all browsers -->
<link rel="stylesheet" href="/assets/css/main.css">
<link rel="preload" href="https://cdn.jsdelivr.net/npm/@fortawesome/fontawesome-free@latest/css/all.min.css" as="style" onload="this.onload=null;this.rel='stylesheet'">
<noscript><link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@fortawesome/fontawesome-free@latest/css/all.min.css"></noscript>



    <!-- start custom head snippets -->

<link rel="apple-touch-icon" sizes="180x180" href="/apple-touch-icon.png">
<link rel="icon" type="image/png" sizes="32x32" href="/favicon-32x32.png">
<link rel="icon" type="image/png" sizes="16x16" href="/favicon-16x16.png">
<link rel="manifest" href="/site.webmanifest">
<link rel="mask-icon" href="/safari-pinned-tab.svg" color="#5bbad5">
<link rel="shortcut icon" href="/favicon.ico">
<meta name="msapplication-TileColor" content="#da532c">
<meta name="msapplication-config" content="/browserconfig.xml">
<meta name="theme-color" content="#ffffff">

<link href="https://fonts.googleapis.com/css?family=Raleway:300,400,600&display=swap" rel="stylesheet">

<link rel="stylesheet" href="/assets/css/Fork-Awesome/css/fork-awesome.min.css">
<link rel="stylesheet" href="/assets/css/Fork-Awesome/css/v5-compat.css">

<!-- Apply specific styles at elements containing a specific text -->
<script type="text/javascript" >
    window.onload = function () {
        getElementsByContains = function(str,elmtTagName="*",node=document){
            var elms = document.evaluate( "//"+elmtTagName+"[contains(., '" +str+ "')]" ,node, null, XPathResult.UNORDERED_NODE_SNAPSHOT_TYPE, null ),
                nodeSet = [];
            for ( var i=0 ; i < elms.snapshotLength; i++ ){
                nodeSet.push(elms.snapshotItem(i));
            }
            return nodeSet;
        }
        getElementsByContains('Donate','a')[0].style.color ='green';
}
</script>

<meta name="flattr:id" content="ol0vo0">

<!-- insert favicons. use https://realfavicongenerator.net/ -->

<!-- end custom head snippets -->

  </head>

  <body class="layout--single">
    <nav class="skip-links">
  <ul>
    <li><a href="#site-nav" class="screen-reader-shortcut">Skip to primary navigation</a></li>
    <li><a href="#main" class="screen-reader-shortcut">Skip to content</a></li>
    <li><a href="#footer" class="screen-reader-shortcut">Skip to footer</a></li>
  </ul>
</nav>

    

<div class="masthead">
  <div class="masthead__inner-wrap">
    <div class="masthead__menu">
      <nav id="site-nav" class="greedy-nav">
        
          <a class="site-logo" href="/"><img src="/yuleremmina.png" alt="Remmina logo"></a>
        
        <a class="site-title" href="/">
          Remmina
          
        </a>
        <ul class="visible-links"><li class="masthead__menu-item">
              <a href="/authors/">About</a>
            </li><li class="masthead__menu-item">
              <a href="/screenshots/">Discover</a>
            </li><li class="masthead__menu-item">
              <a href="/donations/"><i class="fa fa-heart" aria-hidden="true" color="red"></i> Donate</a>
            </li><li class="masthead__menu-item">
              <a href="/contributing-to-remmina/">Join</a>
            </li><li class="masthead__menu-item">
              <a href="/blog/">News</a>
            </li><li class="masthead__menu-item">
              <a href="/support/"><i class="fa fa-life-ring" aria-hidden="true"></i></a>
            </li></ul>
        
        <button class="search__toggle" type="button">
          <span class="visually-hidden">Toggle search</span>
          <svg class="icon" width="16" height="16" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 15.99 16">
            <path d="M15.5,13.12L13.19,10.8a1.69,1.69,0,0,0-1.28-.55l-0.06-.06A6.5,6.5,0,0,0,5.77,0,6.5,6.5,0,0,0,2.46,11.59a6.47,6.47,0,0,0,7.74.26l0.05,0.05a1.65,1.65,0,0,0,.5,1.24l2.38,2.38A1.68,1.68,0,0,0,15.5,13.12ZM6.4,2A4.41,4.41,0,1,1,2,6.4,4.43,4.43,0,0,1,6.4,2Z" transform="translate(-.01)"></path>
          </svg>
        </button>
        
        <button class="greedy-nav__toggle hidden" type="button">
          <span class="visually-hidden">Toggle menu</span>
          <div class="navicon"></div>
        </button>
        <ul class="hidden-links hidden"></ul>
      </nav>
    </div>
  </div>
</div>


    <div class="initial-content">
      





<div id="main" role="main">
  
  <div class="sidebar sticky">
  


<div itemscope itemtype="https://schema.org/Person" class="h-card">

  

  <div class="author__content">
    <h3 class="author__name p-name" itemprop="name">
      <a class="u-url" rel="me" href="https://remmina.org/" itemprop="url">Remmina Project</a>
    </h3>
    
  </div>

  <div class="author__urls-wrapper">
    <button class="btn btn--inverse">Follow</button>
    <ul class="author__urls social-icons">
      

      

      

      

      

      
        <li>
          <a href="https://twitter.com/RemminaProject" itemprop="sameAs" rel="nofollow noopener noreferrer me">
            <i class="fab fa-fw fa-twitter-square" aria-hidden="true"></i><span class="label">Twitter</span>
          </a>
        </li>
      

      

      

      

      

      

      

      

      
        <li>
          <a href="https://gitlab.com/Remmina/Remmina" itemprop="sameAs" rel="nofollow noopener noreferrer me">
            <i class="fab fa-fw fa-gitlab" aria-hidden="true"></i><span class="label">GitLab</span>
          </a>
        </li>
      

      

      

      

      

      

      

      
        
          <li>
            <a href="https://www.youtube.com/channel/UC7mJGLFkx5nZx8cVG4EOrGA" itemprop="sameAs" rel="nofollow noopener noreferrer me">
              <i class="fab fa-fw fa-youtube" aria-hidden="true"></i><span class="label">YouTube</span>
            </a>
          </li>
        
      

      

      

      

      

      

      <!--
  <li>
    <a href="http://link-to-whatever-social-network.com/user/" itemprop="sameAs" rel="nofollow noopener noreferrer me">
      <i class="fas fa-fw" aria-hidden="true"></i> Custom Social Profile Link
    </a>
  </li>
-->
    </ul>
  </div>
</div>

  
  </div>



  <article class="page" itemscope itemtype="https://schema.org/CreativeWork">
    <meta itemprop="headline" content="News">
    
    
    

    <div class="page__inner-wrap">
      
        <header>
          <h1 id="page-title" class="page__title" itemprop="headline">
            <a href="https://remmina.org/news/remmina_news.php" itemprop="url">News
</a>
          </h1>
          


        </header>
      

      <section class="page__content" itemprop="text">
        
        

        
      </section>

      <footer class="page__meta">
        
        


        

      </footer>

      

      
    </div>

    
  </article>

  
  
</div>

      
    </div>

    
      <div class="search-content">
        <div class="search-content__inner-wrap"><form class="search-content__form" onkeydown="return event.key != 'Enter';" role="search">
    <label class="sr-only" for="search">
      Type something to search for…
    </label>
    <input type="search" id="search" class="search-input" tabindex="-1" placeholder="Type something to search for…" />
  </form>
  <div id="results" class="results"></div></div>

      </div>
    

    <div id="footer" class="page__footer">
      <footer>
        <!-- start custom footer snippets -->
<!-- end custom footer snippets -->

        <div class="page__footer-follow">
  <ul class="social-icons">
    
      <li><strong>Follow:</strong></li>
    

    
      
        
          <li><a href="https://gitlab.com/Remmina/Remmina" rel="nofollow noopener noreferrer"><i class="fab fa-fw fa-gitlab" aria-hidden="true"></i> GitLab</a></li>
        
      
        
          <li><a href="https://hosted.weblate.org/projects/remmina/remmina/" rel="nofollow noopener noreferrer"><i class="fab fa-fw fa-weblate" aria-hidden="true"></i> Weblate</a></li>
        
      
        
          <li><a href="https://web.libera.chat/?nick=remminer|?#remmina" rel="nofollow noopener noreferrer"><i class="fa fa-fw fa-commenting" aria-hidden="true"></i> IRC</a></li>
        
      
        
          <li><a href="https://riot.im/app/#/group/+remmina:matrix.org" rel="nofollow noopener noreferrer"><i class="fa fa-fw fa-matrix-org" aria-hidden="true"></i> Matrix</a></li>
        
      
        
          <li><a href="https://www.reddit.com/r/Remmina/" rel="nofollow noopener noreferrer"><i class="fab fa-fw fa-reddit" aria-hidden="true"></i> Reddit</a></li>
        
      
        
          <li><a href="https://twitter.com/RemminaProject" rel="nofollow noopener noreferrer"><i class="fab fa-fw fa-twitter-square" aria-hidden="true"></i> Twitter</a></li>
        
      
        
          <li><a href="https://gitlab.com/Remmina/Remmina/wikis/home" rel="nofollow noopener noreferrer"><i class="fa fa-fw fa-wikipedia-w" aria-hidden="true"></i> Wiki</a></li>
        
      
    

    
      <li><a href="/feed.xml"><i class="fas fa-fw fa-rss-square" aria-hidden="true"></i> Feed</a></li>
    
  </ul>
</div>

<div class="page__footer-copyright">&copy; 2024 <a href="https://remmina.org">Remmina</a>. Powered by <a href="https://jekyllrb.com" rel="nofollow">Jekyll</a> &amp; <a href="https://mademistakes.com/work/minimal-mistakes-jekyll-theme/" rel="nofollow">Minimal Mistakes</a>.</div>

      </footer>
    </div>

    
  <script src="/assets/js/main.min.js"></script>
  <!--<script defer src="https://use.fontawesome.com/releases/v5.8.2/js/all.js" integrity="sha384-DJ25uNYET2XCl5ZF++U8eNxPWqcKohUUBUpKGlNLMchM7q4Wjg2CUpjHLaL8yYPH" crossorigin="anonymous"></script>-->




<script src="/assets/js/lunr/lunr.min.js"></script>
<script src="/assets/js/lunr/lunr-store.js"></script>
<script src="/assets/js/lunr/lunr-en.js"></script>






  </body>
</html>
