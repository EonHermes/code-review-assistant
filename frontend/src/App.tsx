import React, { useState, useEffect } from 'react'
import { ReactFlow, MiniMap, Controls, Background, useNodesState, useEdgesState, Node, Edge } from 'reactflow'
import 'reactflow/dist/style.css'
import './App.css'

interface Module {
  id: string
  name: string
  path: string
  parent: string | null
  children: string[]
  items: Item[]
  documentation?: string
  line_count: number
}

interface Item {
  id: string
  name: string
  kind: string
  visibility: string
  signature?: string
  documentation?: string
  attributes: string[]
  line: number
  children: string[]
}

interface ApiEndpoint {
  path: string
  method: string
  summary?: string
  description?: string
  parameters: any[]
  request_body?: any
  responses: any[]
  tags: string[]
  source_file: string
  line: number
}

interface Statistics {
  total_files: number
  total_lines: number
  total_modules: number
  total_items: number
  total_endpoints: number
  languages: Record<string, number>
  doc_coverage: number
}

interface ProjectInfo {
  name: string
  version?: string
  description?: string
  authors: string[]
  license?: string
  homepage?: string
  repository?: string
  dependencies: any[]
  dev_dependencies: any[]
  language: string
  build_info: { tool: string; target?: string; features: string[] }
}

type Tab = 'overview' | 'modules' | 'api' | 'graph' | 'stats'

function App() {
  const [data, setData] = useState<{
    project: ProjectInfo
    modules: Record<string, Module>
    root_module: string
    api_endpoints: ApiEndpoint[]
    schemas: Record<string, any>
    cross_references: any[]
    statistics: Statistics
  } | null>(null)
  const [activeTab, setActiveTab] = useState<Tab>('overview')
  const [selectedModule, setSelectedModule] = useState<Module | null>(null)
  const [search, setSearch] = useState('')

  useEffect(() => {
    // In real usage, this would fetch from the generated JSON
    // For demo, we'll create sample data
    const sampleData = createSampleData()
    setData(sampleData)
  }, [])

  if (!data) return <div className="loading">Loading documentation...</div>

  const filteredModules = Object.values(data.modules).filter(m =>
    m.name.toLowerCase().includes(search.toLowerCase()) ||
    m.documentation?.toLowerCase().includes(search.toLowerCase())
  )

  return (
    <div className="app">
      <header className="header">
        <div className="header-content">
          <h1>📚 {data.project.name || 'Documentation'}</h1>
          <p>{data.project.description}</p>
          <div className="meta">
            <span className="badge">{data.project.language}</span>
            <span className="badge">{data.statistics.total_modules} modules</span>
            <span className="badge">{data.statistics.total_items} items</span>
            <span className="badge">{data.statistics.doc_coverage.toFixed(1)}% docs</span>
          </div>
        </div>
      </header>

      <div className="main">
        <aside className="sidebar">
          <div className="search">
            <input
              type="text"
              placeholder="Search modules..."
              value={search}
              onChange={(e) => setSearch(e.target.value)}
            />
          </div>

          <nav className="nav">
            <button className={activeTab === 'overview' ? 'active' : ''} onClick={() => setActiveTab('overview')}>Overview</button>
            <button className={activeTab === 'modules' ? 'active' : ''} onClick={() => setActiveTab('modules')}>Modules ({data.statistics.total_modules})</button>
            <button className={activeTab === 'api' ? 'active' : ''} onClick={() => setActiveTab('api')}>API ({data.api_endpoints.length})</button>
            <button className={activeTab === 'graph' ? 'active' : ''} onClick={() => setActiveTab('graph')}>Architecture</button>
            <button className={activeTab === 'stats' ? 'active' : ''} onClick={() => setActiveTab('stats')}>Statistics</button>
          </nav>

          {activeTab === 'modules' && (
            <div className="module-list">
              {filteredModules.map(module => (
                <div
                  key={module.id}
                  className={`module-item ${selectedModule?.id === module.id ? 'selected' : ''}`}
                  onClick={() => setSelectedModule(module)}
                >
                  <strong>{module.name}</strong>
                  <span>{module.items.length} items</span>
                </div>
              ))}
            </div>
          )}
        </aside>

        <main className="content">
          {activeTab === 'overview' && <OverviewTab data={data} />}
          {activeTab === 'modules' && selectedModule && <ModuleDetail module={selectedModule} />}
          {activeTab === 'api' && <ApiTab endpoints={data.api_endpoints} />}
          {activeTab === 'graph' && <GraphTab modules={data.modules} cross_references={data.cross_references} />}
          {activeTab === 'stats' && <StatsTab stats={data.statistics} project={data.project} />}
        </main>
      </div>
    </div>
  )
}

function OverviewTab({ data }: { data: any }) {
  return (
    <div className="tab">
      <h2>Project Overview</h2>
      <section>
        <h3>Information</h3>
        <dl className="grid">
          <div><dt>Name</dt><dd>{data.project.name}</dd></div>
          <div><dt>Version</dt><dd>{data.project.version || 'N/A'}</dd></div>
          <div><dt>Language</dt><dd>{data.project.language}</dd></div>
          <div><dt>Build Tool</dt><dd>{data.project.build_info.tool}</dd></div>
          <div><dt>License</dt><dd>{data.project.license || 'N/A'}</dd></div>
          <div><dt>Repository</dt><dd>{data.project.repository || 'N/A'}</dd></div>
          <div><dt>Authors</dt><dd>{data.project.authors.join(', ') || 'N/A'}</dd></div>
        </dl>
      </section>

      <section>
        <h3>Analytics</h3>
        <div className="stats-grid">
          <div className="stat-card">
            <div className="stat-value">{data.statistics.total_modules}</div>
            <div className="stat-label">Modules</div>
          </div>
          <div className="stat-card">
            <div className="stat-value">{data.statistics.total_items}</div>
            <div className="stat-label">Items</div>
          </div>
          <div className="stat-card">
            <div className="stat-value">{data.statistics.total_files}</div>
            <div className="stat-label">Files</div>
          </div>
          <div className="stat-card">
            <div className="stat-value">{data.statistics.total_lines.toLocaleString()}</div>
            <div className="stat-label">Lines</div>
          </div>
          <div className="stat-card highlight">
            <div className="stat-value">{data.statistics.doc_coverage.toFixed(1)}%</div>
            <div className="stat-label">Documentation Coverage</div>
          </div>
        </div>
      </section>
    </div>
  )
}

function ModuleDetail({ module }: { module: Module }) {
  const [expanded, setExpanded] = useState<string[]>([])

  const toggleExpand = (id: string) => {
    if (expanded.includes(id)) {
      setExpanded(expanded.filter(x => x !== id))
    } else {
      setExpanded([...expanded, id])
    }
  }

  const groupByKind = () => {
    const groups: Record<string, Item[]> = {}
    module.items.forEach(item => {
      if (!groups[item.kind]) groups[item.kind] = []
      groups[item.kind].push(item)
    })
    return groups
  }

  const groups = groupByKind()

  return (
    <div className="tab module-detail">
      <div className="module-header">
        <h2>Module: {module.name}</h2>
        {module.documentation && <p className="module-doc">{module.documentation}</p>}
        <div className="module-meta">
          <span>{module.path}</span>
          <span>{module.line_count} lines</span>
          <span>{module.items.length} items</span>
        </div>
      </div>

      <h3>Contents</h3>
      {Object.entries(groups).map(([kind, items]) => (
        <section key={kind} className="item-group">
          <h4 className="group-header" onClick={() => toggleExpand(kind)}>
            <span className={`arrow ${expanded.includes(kind) ? 'expanded' : ''}`}>▶</span>
            {kind} ({items.length})
          </h4>
          {expanded.includes(kind) && (
            <table className="items-table">
              <thead>
                <tr>
                  <th>Name</th>
                  <th>Visibility</th>
                  <th>Line</th>
                  <th>Documentation</th>
                </tr>
              </thead>
              <tbody>
                {items.map(item => (
                  <tr key={item.id}>
                    <td><code>{item.name}</code></td>
                    <td><span className={`visibility ${item.visibility.toLowerCase()}`}>{item.visibility}</span></td>
                    <td>{item.line}</td>
                    <td>{item.documentation ? '✅' : '❌'}</td>
                  </tr>
                ))}
              </tbody>
            </table>
          )}
        </section>
      ))}
    </div>
  )
}

function ApiTab({ endpoints }: { endpoints: ApiEndpoint[] }) {
  const [methodFilter, setMethodFilter] = useState<string>('all')
  const filtered = methodFilter === 'all' ? endpoints : endpoints.filter(e => e.method === methodFilter)
  const methods = ['all', ...Array.from(new Set(endpoints.map(e => e.method)))]

  return (
    <div className="tab">
      <h2>API Reference</h2>
      <div className="filters">
        {methods.map(method => (
          <button
            key={method}
            className={methodFilter === method ? 'active' : ''}
            onClick={() => setMethodFilter(method)}
          >
            {method === 'all' ? 'All' : method}
          </button>
        ))}
      </div>

      <div className="api-list">
        {filtered.map(endpoint => (
          <div key={`${endpoint.method}-${endpoint.path}`} className="api-card">
            <div className="api-method">{endpoint.method}</div>
            <div className="api-path">{endpoint.path}</div>
            <div className="api-details">
              {endpoint.summary && <p>{endpoint.summary}</p>}
              <div className="api-meta">
                <span>{endpoint.source_file}:{endpoint.line}</span>
                {endpoint.tags.length > 0 && <span>Tags: {endpoint.tags.join(', ')}</span>}
              </div>
            </div>
          </div>
        ))}
      </div>
    </div>
  )
}

function GraphTab({ modules, cross_references }: { modules: Record<string, Module>, cross_references: any[] }) {
  const nodes: Node[] = Object.values(modules).map(m => ({
    id: m.id,
    type: 'default',
    data: { label: m.name },
    position: { x: Math.random() * 400, y: Math.random() * 400 },
    style: { background: '#e0f2fe', border: '1px solid #0369a1', borderRadius: 8, padding: 8 },
  }))

  const edges: Edge[] = cross_references.map((ref, idx) => ({
    id: `edge-${idx}`,
    source: ref.from,
    target: ref.to,
    type: 'smoothstep',
    animated: ref.kind === 'uses',
    style: { stroke: '#64748b' },
    label: ref.kind,
  }))

  return (
    <div className="graph-container">
      <ReactFlow nodes={nodes} edges={edges} fitView>
        <MiniMap />
        <Controls />
        <Background />
      </ReactFlow>
    </div>
  )
}

function StatsTab({ stats, project }: { stats: Statistics, project: ProjectInfo }) {
  return (
    <div className="tab">
      <h2>Statistics</h2>
      <div className="stats-grid">
        <div className="stat-card">
          <div className="stat-value">{stats.total_files}</div>
          <div className="stat-label">Files Scanned</div>
        </div>
        <div className="stat-card">
          <div className="stat-value">{stats.total_lines.toLocaleString()}</div>
          <div className="stat-label">Lines of Code</div>
        </div>
        <div className="stat-card">
          <div className="stat-value">{stats.total_modules}</div>
          <div className="stat-label">Modules</div>
        </div>
        <div className="stat-card">
          <div className="stat-value">{stats.total_items}</div>
          <div className="stat-label">Items</div>
        </div>
        <div className="stat-card highlight">
          <div className="stat-value">{stats.doc_coverage.toFixed(1)}%</div>
          <div className="stat-label">Documentation Coverage</div>
        </div>
      </div>

      <section>
        <h3>Languages</h3>
        <dl className="grid">
          {Object.entries(stats.languages).map(([lang, count]) => (
            <div key={lang}>
              <dt>{lang}</dt>
              <dd>{count} files</dd>
            </div>
          ))}
        </dl>
      </section>

      <section>
        <h3>Dependencies ({project.dependencies.length})</h3>
        {project.dependencies.length > 0 ? (
          <ul className="dependency-list">
            {project.dependencies.map((dep, i) => (
              <li key={i}>
                <strong>{dep.name}</strong>
                <span>{dep.version || 'latest'}</span>
                {dep.is_dev && <span className="dev-tag">dev</span>}
              </li>
            ))}
          </ul>
        ) : (
          <p>No dependencies found.</p>
        )}
      </section>
    </div>
  )
}

function createSampleData() {
  return {
    project: {
      name: "Automated Document Synthesizer",
      version: "0.1.0",
      description: "Automatically generates comprehensive documentation from code repositories",
      authors: ["Eon"],
      license: "MIT",
      repository: "https://github.com/EonHermes/automated-document-synthesizer",
      language: "Rust",
      build_info: { tool: "cargo", features: ["git"] },
      dependencies: [
        { name: "clap", version: "4.5", is_dev: false },
        { name: "serde", version: "1.0", is_dev: false },
        { name: "syn", version: "2.0", is_dev: false },
        { name: "petgraph", version: "0.6", is_dev: false },
      ],
      dev_dependencies: [],
    },
    modules: {
      "root": {
        id: "root",
        name: "root",
        path: ".",
        parent: null,
        children: ["analyzer", "graph", "output", "config", "errors", "rust_parser"],
        items: [],
        documentation: "Project root module",
        line_count: 0,
      },
      "analyzer": {
        id: "analyzer",
        name: "analyzer",
        path: "analyzer.rs",
        parent: "root",
        children: [],
        items: generateSampleItems("analyzer"),
        line_count: 250,
      },
      "graph": {
        id: "graph",
        name: "graph",
        path: "graph.rs",
        parent: "root",
        children: [],
        items: generateSampleItems("graph"),
        line_count: 120,
      },
      "output": {
        id: "output",
        name: "output",
        path: "output.rs",
        parent: "root",
        children: [],
        items: generateSampleItems("output"),
        line_count: 340,
      },
      "config": {
        id: "config",
        name: "config",
        path: "config.rs",
        parent: "root",
        children: [],
        items: generateSampleItems("config"),
        line_count: 180,
      },
      "errors": {
        id: "errors",
        name: "errors",
        path: "errors.rs",
        parent: "root",
        children: [],
        items: generateSampleItems("errors"),
        line_count: 50,
      },
      "rust_parser": {
        id: "rust_parser",
        name: "rust_parser",
        path: "rust_parser.rs",
        parent: "root",
        children: [],
        items: generateSampleItems("rust_parser"),
        line_count: 400,
      },
    },
    root_module: "root",
    api_endpoints: [],
    schemas: {},
    cross_references: [
      { from: "root", to: "analyzer", kind: "uses" },
      { from: "analyzer", to: "graph", kind: "depends" },
      { from: "analyzer", to: "config", kind: "uses" },
      { from: "output", to: "analyzer", kind: "uses" },
      { from: "rust_parser", to: "analyzer", kind: "used_by" },
    ],
    statistics: {
      total_files: 12,
      total_lines: 5423,
      total_modules: 7,
      total_items: 148,
      total_endpoints: 0,
      languages: { Rust: 12 },
      doc_coverage: 73.2,
    },
  }
}

function generateSampleItems(module: string): any[] {
  const base = [
    { kind: 'Struct', visibility: 'Public', hasDocs: true },
    { kind: 'Function', visibility: 'Public', hasDocs: true },
    { kind: 'Enum', visibility: 'Private', hasDocs: false },
    { kind: 'Trait', visibility: 'Public', hasDocs: true },
    { kind: 'Constant', visibility: 'Private', hasDocs: false },
    { kind: 'TypeAlias', visibility: 'Public', hasDocs: true },
  ]

  return base.map((b, i) => ({
    id: `${module}::${b.kind.toLowerCase()}${i}`,
    name: `${module}_${b.kind.toLowerCase()}${i}`,
    kind: b.kind,
    visibility: b.visibility,
    signature: `pub ${b.kind.toLowerCase()} ${module}_${b.kind.toLowerCase()}${i}: impl ${b.kind}`,
    documentation: b.hasDocs ? `This is a sample ${b.kind.toLowerCase()} in ${module}.` : undefined,
    attributes: b.kind === 'Struct' ? ['#[derive(Debug, Clone, Serialize, Deserialize)]'] : [],
    line: 10 + i * 20,
    children: [],
  }))
}

export default App
