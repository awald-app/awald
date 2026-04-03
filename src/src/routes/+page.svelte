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

	interface RowData {
		values: any[];
	}

	let datasetInfo = $state<DatasetMeta | null>(null);
	let tableData = $state<RowData[]>([]);
	let loading = $state(false);
	let tableLoading = $state(false);
	let error = $state<string | null>(null);

	async function loadFile() {
		try {
			loading = true;
			error = null;
			tableData = []; // Clear stale table data

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

				// Load first 100 rows
				await loadTableData(meta.id);
			}
		} catch (err) {
			error = err as string;
			datasetInfo = null; // Clear partial data on error
		} finally {
			loading = false;
		}
	}

	async function loadTableData(datasetId: string) {
		try {
			tableLoading = true;
			const rows: RowData[] = await invoke('get_rows', {
				id: datasetId,
				start: 0,
				end: Math.min(100, datasetInfo?.nrows || 100)
			});
			tableData = rows;
		} catch (err) {
			error = err as string;
			tableData = []; // Clear table data on error
		} finally {
			tableLoading = false;
		}
	}

	function formatValue(value: any): string {
		if (value === null || value === undefined) return '';
		if (typeof value === 'number') {
			return Number.isInteger(value) ? value.toString() : value.toFixed(6);
		}
		return String(value);
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

		{#if tableData.length > 0 || tableLoading}
			<div class="data-table-container">
				<h3>Data Preview {tableLoading ? '(Loading...)' : `(First ${tableData.length} rows)`}</h3>
				<div class="table-wrapper">
					<table class="data-table">
						<thead>
							<tr>
								{#each datasetInfo.schema as column}
									<th>{column.name}</th>
								{/each}
							</tr>
						</thead>
						<tbody>
							{#if tableLoading}
								<tr>
									<td colspan={datasetInfo.ncols} class="loading-cell">
										Loading table data...
									</td>
								</tr>
							{:else if tableData.length === 0}
								<tr>
									<td colspan={datasetInfo.ncols} class="empty-cell">
										No data to display
									</td>
								</tr>
							{:else}
								{#each tableData as row, rowIndex}
									<tr class:even-row={rowIndex % 2 === 0}>
										{#each row.values as value}
											<td>{formatValue(value)}</td>
										{/each}
									</tr>
								{/each}
							{/if}
						</tbody>
					</table>
				</div>
			</div>
		{/if}
	{/if}
</main>

<style>
	main {
		max-width: 1200px;
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

	.data-table-container {
		margin: 20px 0;
		padding: 20px;
		border: 1px solid #ddd;
		border-radius: 4px;
	}

	.data-table-container h3 {
		margin-top: 0;
		color: #333;
	}

	.table-wrapper {
		overflow-x: auto;
		max-height: 600px;
		overflow-y: auto;
		border: 1px solid #ddd;
	}

	.data-table {
		width: 100%;
		border-collapse: collapse;
		font-size: 14px;
	}

	.data-table th {
		background-color: #f5f5f5;
		border: 1px solid #ddd;
		padding: 8px 12px;
		text-align: left;
		font-weight: bold;
		position: sticky;
		top: 0;
		z-index: 10;
	}

	.data-table td {
		border: 1px solid #ddd;
		padding: 6px 12px;
		white-space: nowrap;
		max-width: 200px;
		overflow: hidden;
		text-overflow: ellipsis;
	}

	.data-table .even-row {
		background-color: #fafafa;
	}

	.data-table tr:hover {
		background-color: #f0f8ff;
	}

	.loading-cell, .empty-cell {
		text-align: center;
		font-style: italic;
		color: #666;
		padding: 20px;
	}

	.loading-cell {
		background-color: #f8f9fa;
	}

	.empty-cell {
		background-color: #fafafa;
	}
</style>
