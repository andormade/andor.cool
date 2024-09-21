---
layout: post
title: 'Bookshelf'
date: 2023-07-19 19:00:00 -0100
location: 'Zandvoort, Netherlands'
---

<style>

   .bookshelf {
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        gap: 10px; /* Adjust the gap between items as needed */
    }

    .bookshelf li {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        aspect-ratio: 3 / 4;
        text-align: center;
       // word-break: break-word;
        box-sizing: border-box; 
        position: relative;
        overflow: hidden;
    }

    .bookshelf span {
        width: 100%;
        padding: 0 10px;
        box-sizing: border-box;
    }

    .bookshelf .small {
        font-size: 11px;
        line-height: 11px;
        padding: 5px 10px;
    }

     .bookshelf .big {
        font-size: 33px;
        line-height: 33px;
        padding: 0;
    }

    .bookshelf .upp {
        text-transform: uppercase;
    }

        .bookshelf .low {
        text-transform: lowercase;
    }

    .bookshelf .left {
        text-align: left;
    }

    .bookshelf .right {
        text-align: right;
    }
</style>

<p>Welcome to my virtual bookshelf! The books arranged in no particular order.</p>

<ul class="bookshelf">

<li style="background: #fff;">
<span class="small" style="color: #c60">Nick Bostrom</span>
<span style="color: #000">SUPER INTELLIGENCE</span>
<span class="small" style="color: #000">Paths, Dangers, Strategies</span>
</li>

<li style="background: #9cc;">
<span class="upp" style="color: #f30; background: #fff">The Alignment Problem</span>
<span class="small" style="color: #000; background: #fff">Machine Learning and Human Values</span>
<span class="small upp" style="color: #f30; background: #fff">Brian Christian</span>
</li>

<li style="background: #36f;">
<span  style="color: #fff; background: #003">Being You</span>
<span class="small" style="color: #fff; background: #003">ANIL SETH</span>
<span class="small" style="color: #fff">A New Science of Consciousness</span>
</li>


<li style="background: #fff;">
<span  style="color: #000">A THOUSAND BRAINS</span>
<span class="small" style="color: #f30">A NEW THEORY OF INTELLIGENCE</span>
<span class="small" style="color: #000">JEFF HAWKINS</span>
</li>

<li style="background: #000;">
<span class="small" style="color: #6699cc">WHEN HUMANS TRANSCEND BIOLOGY</span>
<span style="color: #fff">THE SINGULARITY IS NEAR</span>
<span class="small" style="color: #fc6">RAY KURZWEIL</span>
</li>

 <li style="background: #c99;">
 <span class="small" style="color: #000">THE SCIENCE OF THE MIND AND THE MYTH OF THE SELF</span>
 <span style="color: #fff; background: #666">THE EGO TUNNEL</span>
 <span class="small" style="color: #000">THOMAS METZINGER</span>
 </li>

<li style="background: #ffc;">
<span class="small upp" style="color: #000">Evolution, AI, and the Five Breakthroughs That Made Our Brains</span> 
<span class="upp" style="color: #000">A Brief History of Intelligence</span>
<span class="small upp" style="color: #000">Max Bennett</span>
</li>

<li style="background: #369;">
<span class="upp left" style="color: #fff">Global Catastrophic Risks</span>
<span class="small upp" style="color: #fff">Nick Bostrom and Milan M. Ćirković</span>
</li>

<li style="background: #fff;">
<span class="upp" style="color: #000">Behave</span>
<span class="upp small" style="color: #000">The Biology of Humans at Our Best and Worst</span>
<span class="upp" style="color: #000">Robert M. Sapolsky</span>
</li>

<li style="background: #036;">
<span class="upp small left" style="color: #cc6">Max Tegmark</span>
<span class="upp" style="color: #fff">Our Mathematical Universe</span>
<span class="upp small right" style="color: #cc6">My Quest for the Ultimate Nature of Reality</span>
</li>

<li style="background: #633;">
<span class="upp" style="color: #fff">The Order of Time</span>
<span class="upp" style="color: #fff">Carlo Rovelli</span>
</li>

<li style="background: #336;">
<span class="upp" style="color: #fff">Life 3.0</span> 
<span class="upp small" style="color: #fff">Being Human in the Age of Artificial Intelligence</span>
<span class="upp small" style="color: #fff">Max Tegmark</span>
</li>

<li style="background: #033;">
<span class="upp" style="color: #fff">Alien Information Theory</span> 
<span class="small" style="color: #fff">Psychedelic Drug Technologies and the Cosmic Game</span> 
<span class="upp small" style="color: #fff">Andrew R. Gallimore</span> 
</li>

<li style="background: #c33;">
<span class="upp left" style="color: #fff">Complexity</span> 
<span class="upp small left" style="color: #000; background: #fff">The Emerging Science at the Edge of Order and Chaos</span> 
<span class="upp small left" style="color: #c33; background: #fff">M. Mitchell Waldrop</span> 
</li>

<li style="background: #ffc;">
<span class="right" style="color: #000">The Man from the Future</span>
<span class="small" style="color: #000">The Visionary Ideas of John von Neumann</span>
<span class="left" style="color: #000">Ananyo Bhattacharya</span>
</li>

<li style="background: #fff;">
<span class="upp" style="color: #633">Richard Dawkins</span>
<span class="upp" style="color: #000">The Selfish Gene</span>
</li>

<li style="background: #fc6;">
<span class="upp small" style="color: #336">Rick Hanson, PH.D. with Richard Mendius, MD</span>
<span style="color: #fff; background: #366">Buddha's Brain</span>
<span class="low small" style="color: #336">The Practical Neuroscience of Happiness, Love & Wisdom</span>
</li>

<li style="background: #cc9;">
<span class="upp small" style="color: #fff">Thomas Metzinger</span>
<span class="upp" style="color: #fff; background: #996">Being No One</span>
<span class="upp small" style="color: #fff">The Self-Model Theory of Subjectivity</span>
</li>

<li style="background: #fc9;">
<span style="color: #630">The Most Human Human</span>
<span class="small" style="color: #000">What Talking with Computers Teaches Us About What It Means to Be Alive</span>
<span class="small" style="color: #630">Brian Christian</span>
</li>

<li style="background: #000;">
<span class="upp" style="color: #fff">Robert M Saplosky</span>
<span style="color: #fff">Determined</span>
<span class="small upp" style="color: #fff">A Science of Life without Free Will</span>
</li>

<li style="background: #fff;">
<span style="color: #933">Monkeyluv</span>
<span class="small upp" style="color: #000">And Other Essays on Our Lives as Animals</span>
<span class="upp" style="color: #000">Robert M. Saplosky</span>
</li>

<li style="background: #cc6;">
<span class="upp" style="color: #000; background: #ffc">Aldous Huxley</span>
<span class="small upp" style="color: #000; background: #ffc">The Doors of Perception</span>
<span class="small upp" style="color: #000; background: #ffc">Heaven and Hell</span>
</li>

<li style="background: #396;">
<span style="color: #fff">Nuclear War</span>
<span style="color: #fff">Annie Jacobsen</span>
</li>

<li style="background: #f33;">
<span class="upp" style="color: #000">Annie Jacobsen</span>
<span class="upp big" style="color: #000">Operation Paperclip</span>
<span class="upp left small" style="color: #000">The Secret Intelligence Program that Brought Nazi Scientists to America</span>
</li>

<li style="background: #000;">
<span class="upp big" style="color: #fff">Billions & Billions</span>
<span class="small" style="color: #000; background: #fff">Thoughts on Life and Death at the Brink of the Millennium</span>
<span class="upp" style="color: #000; background: #fff">Carl Sagan</span>
</li>

<li style="background: #000">
<span class="upp big" style="color: #c63; background: #fff">Carl Sagan</span>
<span class="upp" style="color: #930; background: #c63">The Demon-Haunted World</span>
<span class="upp" style="color: #c63">Science as a Candle in the Dark</span>
</li>

</ul>