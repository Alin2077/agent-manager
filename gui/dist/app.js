// Tauri v2 IPC
function invoke(cmd, args = {}) {
  const t = window.__TAURI__;
  if (!t) throw new Error('Tauri not loaded');
  if (t.invoke) return t.invoke(cmd, args);
  if (t.core?.invoke) return t.core.invoke(cmd, args);
  throw new Error('No invoke API found');
}

// ═══════════════════════════════════════════
// Error Log System
// ═══════════════════════════════════════════

const errorLog = [];

function logError(category, message, detail = '') {
  const entry = {
    time: new Date().toLocaleString(),
    category,
    message: String(message),
    detail: String(detail)
  };
  errorLog.push(entry);
  updateErrorIndicator();
  return entry;
}

function updateErrorIndicator() {
  const el = document.getElementById('err-indicator');
  if (el) {
    const count = errorLog.length;
    el.style.display = count > 0 ? '' : 'none';
    el.textContent = `⚠️ 错误日志 (${count})`;
  }
}

function openErrorLog() {
  const modal = document.createElement('div');
  modal.className = 'modal-overlay';

  const grouped = {};
  for (const e of errorLog) {
    if (!grouped[e.category]) grouped[e.category] = [];
    grouped[e.category].push(e);
  }

  const entries = Object.entries(grouped).map(([cat, items]) => `
    <div class="err-group">
      <div class="err-group-title">📂 ${cat} (${items.length})</div>
      ${items.map((e, i) => `
        <div class="err-item">
          <div class="err-time">${e.time}</div>
          <div class="err-msg" id="err-msg-${cat}-${i}">${escHtml(e.message)}</div>
          ${e.detail ? `<div class="err-detail" id="err-detail-${cat}-${i}">${escHtml(e.detail)}</div>` : ''}
          <button class="btn-sm err-copy" onclick="copyErr('${cat}',${i})">📋 复制</button>
        </div>
      `).join('')}
    </div>
  `).join('');

  modal.innerHTML = `
    <div class="modal err-log-modal">
      <div style="display:flex;justify-content:space-between;align-items:center;margin-bottom:16px">
        <h3 style="margin:0">📋 错误日志 (${errorLog.length})</h3>
        <div>
          <button class="btn-sm" onclick="copyAllErrors()" style="margin-right:8px">📋 复制全部</button>
          <button class="btn-sm" onclick="clearErrors()">🗑 清空</button>
        </div>
      </div>
      <div class="err-log-body">${entries || '<div class="empty">暂无错误记录</div>'}</div>
      <div class="form-actions" style="margin-top:12px">
        <button class="btn" onclick="this.closest('.modal-overlay').remove()">关闭</button>
      </div>
    </div>
  `;
  document.body.appendChild(modal);
  modal.addEventListener('click', (e) => { if (e.target === modal) modal.remove(); });
}

function escHtml(s) { return String(s).replace(/&/g,'&amp;').replace(/</g,'&lt;').replace(/>/g,'&gt;'); }

function copyErr(cat, i) {
  const e = errorLog.find((_, idx) => {
    let count = 0;
    for (const kv of Object.entries(groupErrors())) {
      if (kv[0] === cat) return count + i === errorLog.indexOf(e);
      count += kv[1].length;
    }
    return false;
  });
  // Simpler approach:
  const items = [];
  for (const g of Object.values(groupErrors())) items.push(...g);
  const entry = items.find((_, idx) => idx === getGlobalErrIdx(cat, i));
  if (!entry) return;
  const text = `[${entry.time}] ${entry.category}: ${entry.message}${entry.detail ? '\n' + entry.detail : ''}`;
  navigator.clipboard.writeText(text).then(() => toast('已复制到剪贴板'));
}

function groupErrors() {
  const g = {};
  for (const e of errorLog) {
    if (!g[e.category]) g[e.category] = [];
    g[e.category].push(e);
  }
  return g;
}

function getGlobalErrIdx(cat, i) {
  let count = 0;
  for (const [c, items] of Object.entries(groupErrors())) {
    if (c === cat) return count + i;
    count += items.length;
  }
  return 0;
}

function copyAllErrors() {
  const text = errorLog.map(e =>
    `[${e.time}] ${e.category}: ${e.message}${e.detail ? '\n  详情: ' + e.detail : ''}`
  ).join('\n---\n');
  navigator.clipboard.writeText(text).then(() => toast('已复制全部错误日志'));
}

function clearErrors() {
  errorLog.length = 0;
  updateErrorIndicator();
  toast('错误日志已清空');
}

// Wrapper: log errors automatically
function logAndToast(category, msg, type = 'error') {
  logError(category, msg);
  toast(msg, type);
}

// ═══════════════════════════════════════════
// i18n
// ═══════════════════════════════════════════

const L = {
  'zh-CN': {
    agents: 'Agents', scan: '🔍 扫描已安装的 Agent', noAgents: '未检测到 Agent，点击上方按钮扫描',
    profiles: 'Profiles', newProfile: '+ 新建 Profile', noProfiles: '还没有 Profile',
    launch: '🚀 启动', edit: '编辑', del: '删除', setDefault: '设默认',
    skills: 'Skills', scanSkills: '🔍 扫描 Skills', noSkills: '还没有 Skill',
    settings: '设置', checkUpdate: '🔄 检查更新', language: '语言',
    langZh: '中文', langEn: 'English', scanSkillHint: '选择一个 Agent 扫描其 Skills',
    selectAgent: '选择 Agent', save: '保存', cancel: '取消',
    updated: '已更新', created: '已创建', deleted: '已删除',
    checking: '检查中...', latest: '已是最新版本', updateFailed: '检查更新失败',
  },
  'en-US': {
    agents: 'Agents', scan: '🔍 Scan Installed Agents', noAgents: 'No agents detected.',
    profiles: 'Profiles', newProfile: '+ New Profile', noProfiles: 'No profiles yet',
    launch: '🚀 Launch', edit: 'Edit', del: 'Delete', setDefault: 'Set Default',
    skills: 'Skills', scanSkills: '🔍 Scan Skills', noSkills: 'No skills yet',
    settings: 'Settings', checkUpdate: '🔄 Check for Updates', language: 'Language',
    langZh: '中文', langEn: 'English', scanSkillHint: 'Select an agent to scan its skills',
    selectAgent: 'Select Agent', save: 'Save', cancel: 'Cancel',
    updated: 'Updated', created: 'Created', deleted: 'Deleted',
    checking: 'Checking...', latest: 'You are up to date', updateFailed: 'Update check failed',
  }
};

let lang = 'zh-CN';
function t(key) { return L[lang]?.[key] || L['zh-CN'][key] || key; }

async function initLang() {
  try { lang = await invoke('get_language'); } catch (e) { logError('初始化', '加载语言设置失败', e); }
}

// ═══════════════════════════════════════════
// Navigation
// ═══════════════════════════════════════════

document.querySelectorAll('.nav-btn').forEach(btn => {
  btn.addEventListener('click', () => {
    document.querySelectorAll('.nav-btn').forEach(b => b.classList.remove('active'));
    btn.classList.add('active');
    loadPage(btn.dataset.page);
  });
});

function loadPage(page) {
  const content = document.getElementById('content');
  content.innerHTML = '';
  switch (page) {
    case 'agents': renderAgents(); break;
    case 'profiles': renderProfiles(); break;
    case 'skills': renderSkills(); break;
    case 'settings': renderSettings(); break;
  }
}

// ═══════════════════════════════════════════
// Toast
// ═══════════════════════════════════════════

function toast(msg, type = 'success') {
  const el = document.createElement('div');
  el.className = `toast toast-${type}`;
  el.textContent = msg;
  document.body.appendChild(el);
  setTimeout(() => el.remove(), 3000);
}

// ═══════════════════════════════════════════
// AGENTS PAGE
// ═══════════════════════════════════════════

let cachedAgents = [];

async function renderAgents() {
  const content = document.getElementById('content');
  content.innerHTML = `
    <div class="page-title">📡 ${t('agents')}</div>
    <button class="btn btn-primary" onclick="scanAndShow()">${t('scan')}</button>
    <div id="agent-list" style="margin-top:20px"></div>
  `;
  try {
    cachedAgents = await invoke('load_agents');
    showAgents(cachedAgents);
  } catch (e) {
    logError('Agents', '加载 Agent 缓存失败', e);
    document.getElementById('agent-list').innerHTML = `<div class="empty">加载失败: ${e.message || e}</div>`;
  }
}

async function scanAndShow() {
  const list = document.getElementById('agent-list');
  list.innerHTML = '<div class="empty">扫描中...</div>';
  try {
    cachedAgents = await invoke('scan_agents');
    showAgents(cachedAgents);
  } catch (e) {
    logError('Agents', '扫描 Agent 失败', e);
    list.innerHTML = `<div class="empty">扫描出错: ${e.message || e}</div>`;
  }
}

function showAgents(agents) {
  const list = document.getElementById('agent-list');
  if (!agents || agents.length === 0) {
    list.innerHTML = `<div class="empty">${t('noAgents')}</div>`;
    return;
  }
  list.innerHTML = `
    <table>
      <thead><tr><th>名称</th><th>命令</th><th>格式</th><th>路径</th><th>版本</th></tr></thead>
      <tbody>
        ${agents.map(a => `
          <tr>
            <td><strong>${a.name}</strong></td>
            <td><code>${a.binary}</code></td>
            <td>${a.formats.join(', ')}</td>
            <td style="font-size:11px;color:var(--text-dim)">${a.path}</td>
            <td>${a.version || '-'}</td>
          </tr>
        `).join('')}
      </tbody>
    </table>
  `;
}

// ═══════════════════════════════════════════
// PROFILES PAGE
// ═══════════════════════════════════════════

async function renderProfiles() {
  const content = document.getElementById('content');
  content.innerHTML = `
    <div class="page-title">⚙️ ${t('profiles')}</div>
    <button class="btn btn-primary" onclick="openProfileModal()">${t('newProfile')}</button>
    <div id="profile-list" style="margin-top:20px"></div>
  `;
  refreshProfileList();
}

async function refreshProfileList() {
  const list = document.getElementById('profile-list');
  try {
    const [profiles, defaultName] = await Promise.all([
      invoke('list_profiles'), invoke('get_default_profile'),
    ]);
    if (profiles.length === 0) {
      list.innerHTML = `<div class="empty">${t('noProfiles')}，点击上方按钮创建</div>`;
      return;
    }
    list.innerHTML = profiles.map(p => `
      <div class="card">
        <div class="card-row">
          <div style="flex:1">
            <div class="card-title">${p.name} ${p.name === defaultName ? '⭐' : ''}</div>
            <div class="card-meta">${p.agent_name || p.agent || '通用'} | ${p.format} | ${p.model}</div>
            <div class="card-meta">${p.base_url}</div>
          </div>
          <button class="btn btn-primary btn-sm" onclick="doLaunch('${p.agent}', '${p.name}')">${t('launch')}</button>
          <button class="btn-sm" onclick="openProfileModal('${p.name}')">${t('edit')}</button>
          <button class="btn-sm" style="color:var(--danger)" onclick="delProfile('${p.name}')">${t('del')}</button>
          ${p.name !== defaultName ? `<button class="btn-sm" onclick="setDefaultProfile('${p.name}')">${t('setDefault')}</button>` : ''}
        </div>
      </div>
    `).join('');
  } catch (e) {
    logError('Profiles', '加载 Profile 列表失败', e);
    list.innerHTML = `<div class="empty">加载出错: ${e.message || e}</div>`;
  }
}

async function doLaunch(agent, profile) {
  if (!agent) { logAndToast('启动', '未关联 Agent'); return; }
  try {
    const code = await invoke('launch_agent', { agent, profile });
    if (code !== 0) logAndToast('启动', `Agent 退出码: ${code}`);
  } catch (e) {
    logError('启动', `启动 ${agent} 失败`, e);
    logAndToast('启动', `启动失败: ${e.message || e}`);
  }
}

async function openProfileModal(name = null) {
  const isEdit = !!name;
  if (cachedAgents.length === 0) {
    try { cachedAgents = await invoke('load_agents'); } catch (_) {}
  }

  const modal = document.createElement('div');
  modal.className = 'modal-overlay';
  const agentOpts = cachedAgents.map(a => `<option value="${a.binary}">${a.name} (${a.binary})</option>`).join('');

  modal.innerHTML = `
    <div class="modal">
      <h3>${isEdit ? '编辑 Profile' : '新建 Profile'}</h3>
      ${isEdit ? '' : '<div class="form-group"><label>名称</label><input id="pf-name" placeholder="my-profile"></div>'}
      <div class="form-group"><label>关联 Agent</label>
        <select id="pf-agent" onchange="onAgentChange()">
          <option value="">-- 选择 Agent --</option>${agentOpts}
        </select></div>
      <div class="form-group"><label>格式</label>
        <select id="pf-format">
          <option value="openai">openai</option><option value="claude-code">claude-code</option><option value="custom">custom</option>
        </select></div>
      <div class="form-group"><label>Base URL</label><input id="pf-url" placeholder="https://api.openai.com/v1"></div>
      <div class="form-group"><label>模型名称</label><input id="pf-model" placeholder="gpt-4o"></div>
      <div class="form-group"><label>API Key</label><input id="pf-key" placeholder="sk-... 或 $ENV_VAR"></div>
      <div class="form-group"><label>Max Tokens</label><input id="pf-tokens" placeholder="4096"></div>
      <div class="form-actions">
        <button class="btn btn-primary" id="pf-save">${t('save')}</button>
        <button class="btn" onclick="this.closest('.modal-overlay').remove()">${t('cancel')}</button>
      </div>
    </div>
  `;
  document.body.appendChild(modal);

  window.onAgentChange = () => {
    const binary = document.getElementById('pf-agent').value;
    const agent = cachedAgents.find(a => a.binary === binary);
    if (agent) {
      if (agent.formats.some(f => f === 'claude-code')) document.getElementById('pf-format').value = 'claude-code';
      else if (agent.formats.some(f => f === 'openai')) document.getElementById('pf-format').value = 'openai';
      if (agent.formats.some(f => f === 'claude-code')) document.getElementById('pf-url').value = 'https://api.anthropic.com/v1';
      else if (agent.formats.some(f => f === 'openai')) document.getElementById('pf-url').value = 'https://api.openai.com/v1';
    }
  };

  if (isEdit) {
    try {
      const p = await invoke('get_profile', { name });
      if (p) {
        document.getElementById('pf-agent').value = p.agent || '';
        document.getElementById('pf-format').value = p.format;
        document.getElementById('pf-url').value = p.base_url;
        document.getElementById('pf-model').value = p.model;
        document.getElementById('pf-key').value = p.api_key;
        if (p.max_tokens) document.getElementById('pf-tokens').value = p.max_tokens;
      }
    } catch (e) { logError('Profiles', '加载 Profile 详情失败', e); }
  }

  document.getElementById('pf-save').addEventListener('click', async () => {
    const binary = document.getElementById('pf-agent').value;
    const agent = cachedAgents.find(a => a.binary === binary);
    const profile = {
      name: isEdit ? name : document.getElementById('pf-name').value,
      agent: binary, agent_name: agent ? agent.name : '',
      format: document.getElementById('pf-format').value,
      base_url: document.getElementById('pf-url').value,
      model: document.getElementById('pf-model').value,
      api_key: document.getElementById('pf-key').value,
      max_tokens: parseInt(document.getElementById('pf-tokens').value) || null,
      extra: {},
    };
    try {
      await invoke('save_profile', { profile });
      modal.remove();
      toast(isEdit ? `"${profile.name}" ${t('updated')}` : `"${profile.name}" ${t('created')}`);
      refreshProfileList();
    } catch (e) {
      logError('Profiles', `保存 Profile 失败`, e);
      logAndToast('Profiles', `保存失败: ${e.message || e}`);
    }
  });
  modal.addEventListener('click', (e) => { if (e.target === modal) modal.remove(); });
}

async function delProfile(name) {
  if (!confirm(`确认删除 Profile "${name}"？`)) return;
  try {
    await invoke('delete_profile', { name });
    toast(`"${name}" ${t('deleted')}`);
    refreshProfileList();
  } catch (e) { logError('Profiles', '删除 Profile 失败', e); logAndToast('Profiles', `删除失败: ${e.message || e}`); }
}

async function setDefaultProfile(name) {
  try {
    await invoke('set_default_profile', { name });
    toast(`默认 Profile = "${name}"`);
    refreshProfileList();
  } catch (e) { logError('Profiles', '设置默认 Profile 失败', e); }
}

// ═══════════════════════════════════════════
// SKILLS PAGE
// ═══════════════════════════════════════════

async function renderSkills() {
  if (cachedAgents.length === 0) {
    try { cachedAgents = await invoke('load_agents'); } catch (_) {}
  }
  const agentOpts = cachedAgents.map(a => `<option value="${a.binary}">${a.name}</option>`).join('');
  const content = document.getElementById('content');
  content.innerHTML = `
    <div class="page-title">📚 ${t('skills')}</div>
    <div class="card" style="margin-bottom:20px">
      <div class="card-row">
        <div class="form-group" style="flex:1;margin:0">
          <label>${t('scanSkillHint')}</label>
          <select id="sk-agent-select"><option value="">-- ${t('selectAgent')} --</option>${agentOpts}</select>
        </div>
        <button class="btn btn-primary btn-sm" onclick="scanSkills()" style="margin-top:18px">${t('scanSkills')}</button>
      </div>
    </div>
    <button class="btn btn-primary" onclick="openSkillModal()">+ 新建 Skill</button>
    <div id="skill-list" style="margin-top:20px"></div>
  `;
  refreshSkillList();
}

async function scanSkills() {
  const sel = document.getElementById('sk-agent-select');
  if (!sel || !sel.value) { logAndToast('Skills', '请先选择 Agent'); return; }
  try {
    const scanned = await invoke('scan_skills', { agent: sel.value });
    if (scanned.length === 0) { toast('未找到 Skills'); return; }
    for (const s of scanned) { try { await invoke('save_skill', { skill: s }); } catch (_) {} }
    toast(`已导入 ${scanned.length} 个 Skills`);
    refreshSkillList();
  } catch (e) { logError('Skills', `扫描 Skills 失败`, e); logAndToast('Skills', `扫描失败: ${e.message || e}`); }
}

async function refreshSkillList() {
  const list = document.getElementById('skill-list');
  if (!list) return;
  try {
    const skills = await invoke('list_skills');
    if (skills.length === 0) { list.innerHTML = `<div class="empty">${t('noSkills')}</div>`; return; }
    const groups = {};
    for (const s of skills) {
      const key = s.agent || '通用';
      if (!groups[key]) groups[key] = [];
      groups[key].push(s);
    }
    list.innerHTML = Object.entries(groups).map(([agent, items]) => `
      <div style="margin-bottom:16px">
        <div style="font-size:12px;color:var(--accent);margin-bottom:6px">🤖 ${agent}</div>
        ${items.map(s => `
          <div class="card" style="margin-bottom:8px">
            <div><div class="card-title">${s.name} <span style="font-weight:400;color:var(--text-dim)">v${s.version}</span></div>
            <div class="card-meta">${s.description}</div></div>
            <div style="margin-top:8px;display:flex;gap:8px">
              <button class="btn-sm" onclick="openSkillModal('${s.name}')">${t('edit')}</button>
              <button class="btn-sm" style="color:var(--danger)" onclick="delSkill('${s.name}')">${t('del')}</button>
            </div>
          </div>
        `).join('')}
      </div>
    `).join('');
  } catch (e) { logError('Skills', '加载 Skill 列表失败', e); }
}

function openSkillModal(name = null) {
  const isEdit = !!name;
  const modal = document.createElement('div');
  modal.className = 'modal-overlay';
  modal.innerHTML = `
    <div class="modal">
      <h3>${isEdit ? '编辑 Skill' : '新建 Skill'}</h3>
      ${isEdit ? '' : '<div class="form-group"><label>名称</label><input id="sk-name" placeholder="my-skill"></div>'}
      <div class="form-group"><label>关联 Agent</label><input id="sk-agent" placeholder="claude"></div>
      <div class="form-group"><label>描述</label><input id="sk-desc" placeholder="一句话描述"></div>
      <div class="form-group"><label>版本</label><input id="sk-ver" value="1.0"></div>
      <div class="form-group"><label>Prompt</label><textarea id="sk-prompt"></textarea></div>
      <div class="form-actions">
        <button class="btn btn-primary" id="sk-save">${t('save')}</button>
        <button class="btn" onclick="this.closest('.modal-overlay').remove()">${t('cancel')}</button>
      </div>
    </div>
  `;
  document.body.appendChild(modal);

  if (isEdit) {
    invoke('get_skill', { name }).then(s => {
      if (s) {
        document.getElementById('sk-agent').value = s.agent || '';
        document.getElementById('sk-desc').value = s.description;
        document.getElementById('sk-ver').value = s.version;
        document.getElementById('sk-prompt').value = s.prompt;
      }
    }).catch(e => logError('Skills', '加载 Skill 详情失败', e));
  }

  document.getElementById('sk-save').addEventListener('click', async () => {
    const skill = {
      name: isEdit ? name : document.getElementById('sk-name').value,
      agent: document.getElementById('sk-agent').value,
      description: document.getElementById('sk-desc').value,
      version: document.getElementById('sk-ver').value,
      prompt: document.getElementById('sk-prompt').value,
    };
    try {
      await invoke('save_skill', { skill });
      modal.remove();
      toast(isEdit ? `"${skill.name}" ${t('updated')}` : `"${skill.name}" ${t('created')}`);
      refreshSkillList();
    } catch (e) { logError('Skills', '保存 Skill 失败', e); logAndToast('Skills', `保存失败: ${e.message || e}`); }
  });
  modal.addEventListener('click', (e) => { if (e.target === modal) modal.remove(); });
}

async function delSkill(name) {
  if (!confirm(`确认删除 Skill "${name}"？`)) return;
  try { await invoke('delete_skill', { name }); toast(`"${name}" ${t('deleted')}`); refreshSkillList(); }
  catch (e) { logError('Skills', '删除 Skill 失败', e); }
}

// ═══════════════════════════════════════════
// SETTINGS PAGE
// ═══════════════════════════════════════════

async function renderSettings() {
  const content = document.getElementById('content');
  content.innerHTML = `
    <div class="page-title">🔧 ${t('settings')}</div>
    <div class="card" style="margin-bottom:16px">
      <div class="card-row">
        <div style="flex:1"><div class="card-title">${t('checkUpdate')}</div>
        <div class="card-meta" id="update-status">--</div></div>
        <button class="btn btn-primary btn-sm" onclick="doCheckUpdate()">${t('checkUpdate')}</button>
      </div>
    </div>
    <div class="card" style="margin-bottom:16px">
      <div class="card-row">
        <div style="flex:1"><div class="card-title">${t('language')}</div></div>
        <select id="lang-select" style="padding:6px 10px;background:var(--bg);border:1px solid var(--border);border-radius:6px;color:var(--text);font-size:13px" onchange="switchLang(this.value)">
          <option value="zh-CN">${t('langZh')}</option><option value="en-US">${t('langEn')}</option>
        </select>
      </div>
    </div>
    <div class="card">
      <div class="card-row">
        <div style="flex:1"><div class="card-title">📋 错误日志</div>
        <div class="card-meta">共 ${errorLog.length} 条记录</div></div>
        <button class="btn-sm" onclick="openErrorLog()">查看</button>
      </div>
    </div>
  `;
  document.getElementById('lang-select').value = lang;
  showVersion();
}

async function showVersion() {
  try {
    const el = document.getElementById('version-label');
    if (el) el.textContent = 'v' + await invoke('get_version');
  } catch (e) { logError('初始化', '获取版本号失败', e); }
}

async function doCheckUpdate() {
  const status = document.getElementById('update-status');
  if (status) status.textContent = t('checking');
  try {
    const msg = await invoke('check_update');
    if (status) status.textContent = msg;
    toast(msg);
  } catch (e) {
    logError('更新', '检查更新失败', e);
    if (status) status.textContent = t('updateFailed');
    logAndToast('更新', `${t('updateFailed')}: ${e.message || e}`);
  }
}

async function switchLang(value) {
  lang = value;
  try { await invoke('save_language', { lang: value }); } catch (e) { logError('设置', '保存语言失败', e); }
  const active = document.querySelector('.nav-btn.active');
  if (active) loadPage(active.dataset.page);
}

// ═══════════════════════════════════════════
// Init
// ═══════════════════════════════════════════

(async () => {
  try {
    await initLang();
    loadPage('agents');
    showVersion();
  } catch (e) {
    logError('初始化', '应用启动失败', e);
    document.getElementById('content').innerHTML = `<div class="empty">启动出错: ${e.message || e}</div>`;
  }
})();
