<script lang="ts">
	import Topbar from "./assets/Topbar.svelte";
	import Title from "./assets/Title.svelte";
</script>

<svelte:head>
	<link href='https://fonts.googleapis.com/css?family=Montserrat' rel='stylesheet'>
</svelte:head>

<main>
	<Topbar />
	<Title />
	<!--<label for="version">Version</label>
	<select name="version" id="version">
		<option value="1.19.2">1.19.2</option>
	</select>-->
</main>

<style>
	:root {
		--darker: rgb(35, 39, 42);
		--dark: rgb(44, 47, 51);
		--light: rgb(153, 170, 181);
		--anim-speed: 150ms;
	}

	main {
		background-color: var(--dark);
		width: 100%;
		height: 100%;
		border-radius: 0 0 10px 10px;
	}
</style>