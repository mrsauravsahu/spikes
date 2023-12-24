import sirv from 'sirv';
import polka from 'polka';
import compression from 'compression';
import * as sapper from '@sapper/server';

const { PORT, HOST, NODE_ENV } = process.env;
const dev = NODE_ENV === 'development';

const serverPath = PORT

polka() // You can also use Express
	.use(
		compression({ threshold: 0 }),
		sirv('static', { dev }),
		sapper.middleware()
	)
	.listen(serverPath, err => {
		if (err) console.log('error', err);
		console.log(`running at : ${serverPath}`)
	});
