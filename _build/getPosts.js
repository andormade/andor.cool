const fs = require('fs').promises;
const parseLiquidTemplateWithFrontMatter = require('./templateParser');

const POSTS_DIR = '_posts';

module.exports = async () => {
	const postFiles = await fs.readdir(POSTS_DIR);
	return await Promise.all(
		postFiles.map(async file => {
			const html = await parseLiquidTemplateWithFrontMatter(`${POSTS_DIR}/${file}`);
			const { mtimeMs } = await fs.stat(`${POSTS_DIR}/${file}`);

			return {
				html,
				mtime: Math.floor(mtimeMs / 1000),
			};
		})
	);
};
