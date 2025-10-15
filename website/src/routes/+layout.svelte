<script lang="ts">
	let { children } = $props();
</script>

<svelte:head>
    <title>Stock Analysis</title>
</svelte:head>

{@render children?.()}
