require('@babel/register')({
	presets: ['@babel/react'],
	extensions: ['.jsx'],
	cache: true,
});

const fs = require('fs').promises;
const getPosts = require('./data/getPosts');
const getPages = require('./data/getPages');
const config = require('./config');
const splitToEqualChunks = require('./utils/splitToEqualChunks');
const createFolders = require('./utils/createFolders');
const CleanCSS = require('clean-css');
const renderPages = require('./render/renderPages');
const renderIndexPages = require('./render/renderIndexPages');
const Post = require('../_layouts/Post.jsx').default;

const rmrf = require('rmrf');

const build = async function ({ clearAll } = {}) {
	const renderTime = Date.now();
	const posts = (await getPosts('./_posts')).map(post => ({ ...post, Component: Post }));
	const pages = await getPages('./_pages');
	//const indexPages = splitToEqualChunks(posts, config.postsPerPage);

	if (clearAll) {
		await rmrf('./public');
		await createFolders(Object.values(config.folders), './public');
	}

	const globalVariables = {
		config,
		posts,
		renderTime,
	};

	let extractedCss = '';

	await renderPages(pages, globalVariables, async ({ html, css, page: { fileName } }) => {
		await fs.writeFile(`public/${fileName.toLowerCase()}.html`, html);
		extractedCss += css;
	});
	// await renderIndexPages(indexPages, globalVariables, async ({ pageNumber, html, css }) => {
	// 	const fileName = pageNumber === 1 ? `public/index.html` : `public/${config.folders.pages}/${pageNumber}.html`;
	// 	await fs.writeFile(fileName, html);
	// 	extractedCss += css;
	// });
	await renderPages(posts, globalVariables, async ({ html, css, page: { fileName } }) => {
		await fs.writeFile(`public/${config.folders.posts}/${fileName}.html`, html);
		extractedCss += css;
	});

	const { styles } = new CleanCSS({ level: 2 }).minify(extractedCss);
	fs.writeFile(`public/style.css`, styles);

	console.log(Date.now() - renderTime + 'ms');
};

build({ clearAll: true });
