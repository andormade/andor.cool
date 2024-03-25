[![Netlify Status](https://api.netlify.com/api/v1/badges/a8bd44af-89f0-4afe-8765-f9cfc38191bf/deploy-status)](https://app.netlify.com/sites/andor/deploys)

# andor.cool

Hello there! 👋 You just found the source code of my personal website. It's powered by a static site generator I wrote in Rust, without using any third-party libraries. This whole project is a playground for me to improve my, let’s say, “rusty” Rust skills. Originally, I wanted to write it in C, but Netlify still doesn't support deploying C projects. :( 

For storing the content, I'm using Markdown files with Front Matter, and with Liquid and Handlebars templates. This approach was inspired by Jekyll, which uses a similar structure for content management.

## Requirements

To run the website locally, make sure you have the following installed:
- Git
- Rust

## Installation

To get the website running on your local machine, follow these steps:

    git clone https://github.com/hngrhorace/andor.cool.git
    cd andor.cool
    cargo run -- generate
    cargo run -- serve
