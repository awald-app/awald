<script lang="ts">
	import { open } from '@tauri-apps/plugin-dialog';
	import { invoke } from '@tauri-apps/api/core';

	interface DatasetMeta {
		id: string;
		filename: string;
		nrows: number;
		ncols: number;
		schema: Array<{ name: string; dtype: string }>;
	}

	let datasetInfo = $state<DatasetMeta | null>(null);
	let loading = $state(false);
	let error = $state<string | null>(null);

	async function loadFile() {
		try {
			loading = true;
			error = null;

			const selected = await open({
				multiple: false,
				filters: [{
					name: 'CSV Files',
					extensions: ['csv']
				}]
			});

			if (selected && typeof selected === 'string') {
				const meta: DatasetMeta = await invoke('load_file', { path: selected });
				datasetInfo = meta;
			}
		} catch (err) {
			error = err as string;
		} finally {
			loading = false;
		}
	}
</script>

<main>
	<h1>Awald - Statistical Computing</h1>

	<div class="controls">
		<button onclick={loadFile} disabled={loading}>
			{loading ? 'Loading...' : 'Load CSV File'}
		</button>
	</div>

	{#if error}
		<div class="error">
			<strong>Error:</strong> {error}
		</div>
	{/if}

	{#if datasetInfo}
		<div class="status-bar">
			{datasetInfo.filename} — {datasetInfo.nrows.toLocaleString()} rows × {datasetInfo.ncols} cols
		</div>

		<div class="dataset-info">
			<h3>Dataset Schema</h3>
			<ul>
				{#each datasetInfo.schema as column}
					<li><strong>{column.name}</strong>: {column.dtype}</li>
				{/each}
			</ul>
		</div>
	{/if}
</main>

<style>
	main {
		max-width: 800px;
		margin: 0 auto;
		padding: 20px;
		font-family: system-ui, sans-serif;
	}

	.controls {
		margin: 20px 0;
	}

	button {
		padding: 10px 20px;
		font-size: 16px;
		background-color: #0066cc;
		color: white;
		border: none;
		border-radius: 4px;
		cursor: pointer;
	}

	button:hover:not(:disabled) {
		background-color: #0052a3;
	}

	button:disabled {
		background-color: #ccc;
		cursor: not-allowed;
	}

	.status-bar {
		margin: 20px 0;
		padding: 10px;
		background-color: #f0f8ff;
		border-left: 4px solid #0066cc;
		font-weight: bold;
	}

	.error {
		margin: 20px 0;
		padding: 10px;
		background-color: #ffe6e6;
		border-left: 4px solid #cc0000;
		color: #cc0000;
	}

	.dataset-info {
		margin: 20px 0;
		padding: 20px;
		border: 1px solid #ddd;
		border-radius: 4px;
	}

	.dataset-info h3 {
		margin-top: 0;
		color: #333;
	}

	.dataset-info ul {
		list-style-type: none;
		padding: 0;
	}

	.dataset-info li {
		padding: 5px 0;
		border-bottom: 1px solid #eee;
	}

	.dataset-info li:last-child {
		border-bottom: none;
	}
</style>
