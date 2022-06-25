import ngrok from 'ngrok'
import chalk from 'chalk'

export default function() {
    const { nuxt } = this;

    if (nuxt.options.dev === 'false') {
        return;
    }

    const options = nuxt.options.ngrok || {};
    const token = process.env.NGROK_TOKEN || options.token;

    let url;

    nuxt.hook('listen', async (server, { port }) => {
        if (token) {
            await ngrok.authtoken(token);
        }
        
        url = await ngrok.connect(port);

        nuxt.options.publicRuntimeConfig.ngrok = { url };

        nuxt.options.cli.badgeMessages.push(
            `Public URL: ${chalk.underline.yellow(url)}`
            );
    });

    nuxt.hook('close', () => {
        url && ngrok.disconnect();
    });
}