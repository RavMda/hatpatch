<script lang="ts">
	import Topbar from "./components/Topbar.svelte";
	import Main from "./components/Main/Main.svelte";
	import Link from "./components/Link.svelte";
</script>

<svelte:head>
	<link href='https://fonts.googleapis.com/css?family=Montserrat' rel='stylesheet'>
</svelte:head>

<main>
	<Topbar />
	<Main />
	<Link />
</main>

<style>
	:root {
		--darker: rgb(35, 39, 42);
		--dark: rgb(44, 47, 51);
		--light: rgb(153, 170, 181);
		--anim-speed: 150ms;
	}

	:global(html){
		user-select: none;
	}

	main {
		background-color: var(--dark);
		width: 100%;
		height: 100%;
		border-radius: 0 0 10px 10px;
	}
</style>