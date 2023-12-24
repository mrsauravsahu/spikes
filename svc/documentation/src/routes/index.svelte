<script context="module">
	export async function preload() {
		const services = [
			`http://${process.env.API_SERVICE_HOST}:${process.env.API_SERVICE_PORT}/`,
			`http://${process.env.PARSER_SERVICE_HOST}:${process.env.PARSER_SERVICE_PORT}/`
		]

		const requestPromises = services.map(
			(serviceUrl) =>
				new Promise(async (resolve) => {
					let connection = true;
					let responseJson = {};

					try {
						const response = await this.fetch(serviceUrl);
						responseJson = await response.json();
					}
					catch(error) {
						connection = false;
					}

					resolve({
						serviceUrl,
						connection,
						body: responseJson,
					});
				})
		);

				const serviceData = await Promise.allSettled(requestPromises);
				return {serviceData};
	}
</script>

<script>
	export let serviceData;
</script>

<h1>Template Monorepo</h1>

<hr />
<h2>Services</h2>
<ul>
	{#each serviceData as service}
		<li>
			<div class="service-url">{service.value.serviceUrl}</div>
			<div>Connection: {service.value.connection ? "Alive": "Could not establish"}</div>
			<div>Reported Service Name: {service.value.body.name ?? "Unknown"}</div>
		</li>
	{/each}
</ul>

<style>
	.service-url {
		font-weight: bold;
	}
</style>