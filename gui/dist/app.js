// Tauri v2 IPC — 兼容不同版本的 API 路径
const __tauri = window.__TAURI__;
const invoke = (cmd, args = {}) => {
  if (!__tauri) throw new Error('Tauri runtime not loaded');
  // v2: core.invoke, 或顶层 invoke（withGlobalTauri: true 时）
  const api = __tauri.core?.invoke || __tauri.invoke;
  if (!api) throw new Error('Tauri invoke not available');
  return api(cmd, args);
};

// ═══════════════════════════════════════════
// i18n
// ═══════════════════════════════════════════

const L = {
  'zh-CN': {
    agents: 'Agents',
    scan: '🔍 扫描已安装的 Agent',
    rescan: '🔄 重新扫描',
    noAgents: '未检测到 Agent，点击上方按钮扫描',
    profiles: 'Profiles',
    newProfile: '+ 新建 Profile',
    noProfiles: '还没有 Profile',
    launch: '🚀 启动',
    edit: '编辑',
    del: '删除',
    setDefault: '设默认',
    skills: 'Skills',
    scanSkills: '🔍 扫描 Skills',
    noSkills: '还没有 Skill',
    settings: '设置',
    checkUpdate: '🔄 检查更新',
    language: '语言',
    langZh: '中文',
    langEn: 'English',
    scanSkillHint: '选择一个 Agent 扫描其 Skills',
    selectAgent: '选择 Agent',
    saving: '保存中...',
    save: '保存',
    cancel: '取消',
    updated: '已更新',
    created: '已创建',
    deleted: '已删除',
    checking: '检查中...',
    latest: '已是最新版本',
    updateFailed: '检查更新失败',
  },
  'en-US': {
    agents: 'Agents',
    scan: '🔍 Scan Installed Agents',
    rescan: '🔄 Rescan',
    noAgents: 'No agents detected. Click the button above to scan.',
    profiles: 'Profiles',
    newProfile: '+ New Profile',
    noProfiles: 'No profiles yet',
    launch: '🚀 Launch',
    edit: 'Edit',
    del: 'Delete',
    setDefault: 'Set Default',
    skills: 'Skills',
    scanSkills: '🔍 Scan Skills',
    noSkills: 'No skills yet',
    settings: 'Settings',
    checkUpdate: '🔄 Check for Updates',
    language: 'Language',
    langZh: '中文',
    langEn: 'English',
    scanSkillHint: 'Select an agent to scan its skills',
    selectAgent: 'Select Agent',
    saving: 'Saving...',
    save: 'Save',
    cancel: 'Cancel',
    updated: 'Updated',
    created: 'Created',
    deleted: 'Deleted',
    checking: 'Checking...',
    latest: 'You are up to date',
    updateFailed: 'Update check failed',
  }
};

let lang = 'zh-CN';

function t(key) {
  return L[lang]?.[key] || L['zh-CN'][key] || key;
}

async function initLang() {
  try { lang = await invoke('get_language'); } catch (e) {
    console.error('initLang failed:', e);
  }
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
  setTimeout(() => el.remove(), 2500);
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
    document.getElementById('agent-list').innerHTML = `<div class="empty">加载失败: ${e}</div>`;
  }
}

async function scanAndShow() {
  const list = document.getElementById('agent-list');
  list.innerHTML = '<div class="empty">扫描中...</div>';
  try {
    cachedAgents = await invoke('scan_agents');
    showAgents(cachedAgents);
  } catch (e) {
    list.innerHTML = '<div class="empty">扫描出错: ' + e + '</div>';
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
      invoke('list_profiles'),
      invoke('get_default_profile'),
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
            <div class="card-meta">
              ${p.agent_name || p.agent || '通用'} | ${p.format} | ${p.model}
            </div>
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
    list.innerHTML = '<div class="empty">加载出错: ' + e + '</div>';
  }
}

async function doLaunch(agent, profile) {
  if (!agent) { toast('请先编辑 Profile 关联 Agent', 'error'); return; }
  try {
    const code = await invoke('launch_agent', { agent, profile });
    if (code === 0) toast('Agent 已退出');
    else toast('Agent 退出码: ' + code, 'error');
  } catch (e) {
    toast('启动失败: ' + e, 'error');
  }
}

async function openProfileModal(name = null) {
  const isEdit = !!name;
  // Load agents for dropdown
  if (cachedAgents.length === 0) {
    try { cachedAgents = await invoke('load_agents'); } catch (_) {}
  }

  const modal = document.createElement('div');
  modal.className = 'modal-overlay';

  const agentOpts = cachedAgents.map(a =>
    `<option value="${a.binary}">${a.name} (${a.binary})</option>`
  ).join('');

  modal.innerHTML = `
    <div class="modal">
      <h3>${isEdit ? '编辑 Profile' : '新建 Profile'}</h3>
      ${isEdit ? '' : '<div class="form-group"><label>名称</label><input id="pf-name" placeholder="my-profile"></div>'}
      <div class="form-group"><label>关联 Agent</label>
        <select id="pf-agent" onchange="onAgentChange()">
          <option value="">-- 选择 Agent --</option>
          ${agentOpts}
        </select>
      </div>
      <div class="form-group"><label>格式</label>
        <select id="pf-format">
          <option value="openai">openai</option>
          <option value="claude-code">claude-code</option>
          <option value="custom">custom</option>
        </select>
      </div>
      <div class="form-group"><label>Base URL</label><input id="pf-url" placeholder="https://api.openai.com/v1"></div>
      <div class="form-group"><label>模型名称</label><input id="pf-model" placeholder="gpt-4o"></div>
      <div class="form-group"><label>API Key</label><input id="pf-key" placeholder="sk-... 或 $ENV_VAR"></div>
      <div class="form-group"><label>Max Tokens（留空=默认）</label><input id="pf-tokens" placeholder="4096"></div>
      <div class="form-actions">
        <button class="btn btn-primary" id="pf-save">${t('save')}</button>
        <button class="btn" onclick="this.closest('.modal-overlay').remove()">${t('cancel')}</button>
      </div>
    </div>
  `;
  document.body.appendChild(modal);

  // auto-fill when agent changes
  window.onAgentChange = () => {
    const binary = document.getElementById('pf-agent').value;
    const agent = cachedAgents.find(a => a.binary === binary);
    if (agent) {
      if (agent.formats.includes('claude-code')) document.getElementById('pf-format').value = 'claude-code';
      else if (agent.formats.includes('openai')) document.getElementById('pf-format').value = 'openai';
      if (agent.formats.includes('claude-code')) document.getElementById('pf-url').value = 'https://api.anthropic.com/v1';
      else if (agent.formats.includes('openai')) document.getElementById('pf-url').value = 'https://api.openai.com/v1';
    }
  };

  if (isEdit) {
    const p = await invoke('get_profile', { name });
    if (p) {
      document.getElementById('pf-agent').value = p.agent || '';
      document.getElementById('pf-format').value = p.format;
      document.getElementById('pf-url').value = p.base_url;
      document.getElementById('pf-model').value = p.model;
      document.getElementById('pf-key').value = p.api_key;
      if (p.max_tokens) document.getElementById('pf-tokens').value = p.max_tokens;
    }
  }

  document.getElementById('pf-save').addEventListener('click', async () => {
    const binary = document.getElementById('pf-agent').value;
    const agent = cachedAgents.find(a => a.binary === binary);
    const profile = {
      name: isEdit ? name : document.getElementById('pf-name').value,
      agent: binary,
      agent_name: agent ? agent.name : '',
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
      toast(isEdit ? `Profile "${profile.name}" ${t('updated')}` : `Profile "${profile.name}" ${t('created')}`);
      refreshProfileList();
    } catch (e) {
      toast('保存失败: ' + e, 'error');
    }
  });

  modal.addEventListener('click', (e) => { if (e.target === modal) modal.remove(); });
}

async function delProfile(name) {
  if (!confirm(`确认删除 Profile "${name}"？`)) return;
  await invoke('delete_profile', { name });
  toast(`"${name}" ${t('deleted')}`);
  refreshProfileList();
}

async function setDefaultProfile(name) {
  await invoke('set_default_profile', { name });
  toast(`默认 Profile = "${name}"`);
  refreshProfileList();
}

// ═══════════════════════════════════════════
// SKILLS PAGE
// ═══════════════════════════════════════════

async function renderSkills() {
  const content = document.getElementById('content');

  if (cachedAgents.length === 0) {
    try { cachedAgents = await invoke('load_agents'); } catch (_) {}
  }

  const agentOpts = cachedAgents.map(a =>
    `<option value="${a.binary}">${a.name}</option>`
  ).join('');

  content.innerHTML = `
    <div class="page-title">📚 ${t('skills')}</div>
    <div class="card" style="margin-bottom:20px">
      <div class="card-row">
        <div class="form-group" style="flex:1;margin:0">
          <label>${t('scanSkillHint')}</label>
          <select id="sk-agent-select">
            <option value="">-- ${t('selectAgent')} --</option>
            ${agentOpts}
          </select>
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
  if (!sel || !sel.value) { toast('请先选择 Agent', 'error'); return; }
  const agent = sel.value;
  try {
    const scanned = await invoke('scan_skills', { agent });
    if (scanned.length === 0) {
      toast(`Agent "${agent}" 没有找到 Skills`);
    } else {
      // Save scanned skills
      for (const s of scanned) {
        try { await invoke('save_skill', { skill: s }); } catch (_) {}
      }
      toast(`已导入 ${scanned.length} 个 Skills`);
      refreshSkillList();
    }
  } catch (e) {
    toast('扫描失败: ' + e, 'error');
  }
}

async function refreshSkillList() {
  const list = document.getElementById('skill-list');
  if (!list) return;
  try {
    const skills = await invoke('list_skills');
    if (skills.length === 0) {
      list.innerHTML = `<div class="empty">${t('noSkills')}</div>`;
      return;
    }
    // Group by agent
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
            <div>
              <div class="card-title">${s.name} <span style="font-weight:400;color:var(--text-dim)">v${s.version}</span></div>
              <div class="card-meta">${s.description}</div>
            </div>
            <div style="margin-top:8px;display:flex;gap:8px">
              <button class="btn-sm" onclick="openSkillModal('${s.name}')">${t('edit')}</button>
              <button class="btn-sm" style="color:var(--danger)" onclick="delSkill('${s.name}')">${t('del')}</button>
            </div>
          </div>
        `).join('')}
      </div>
    `).join('');
  } catch (e) {
    list.innerHTML = '<div class="empty">加载出错: ' + e + '</div>';
  }
}

function openSkillModal(name = null) {
  const isEdit = !!name;
  const modal = document.createElement('div');
  modal.className = 'modal-overlay';
  modal.innerHTML = `
    <div class="modal">
      <h3>${isEdit ? '编辑 Skill' : '新建 Skill'}</h3>
      ${isEdit ? '' : '<div class="form-group"><label>名称</label><input id="sk-name" placeholder="my-skill"></div>'}
      <div class="form-group"><label>关联 Agent（可留空）</label><input id="sk-agent" placeholder="claude"></div>
      <div class="form-group"><label>描述</label><input id="sk-desc" placeholder="一句话描述"></div>
      <div class="form-group"><label>版本</label><input id="sk-ver" value="1.0"></div>
      <div class="form-group"><label>Prompt</label><textarea id="sk-prompt" placeholder="输入 Skill 的完整 prompt..."></textarea></div>
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
    });
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
    } catch (e) {
      toast('保存失败: ' + e, 'error');
    }
  });

  modal.addEventListener('click', (e) => { if (e.target === modal) modal.remove(); });
}

async function delSkill(name) {
  if (!confirm(`确认删除 Skill "${name}"？`)) return;
  await invoke('delete_skill', { name });
  toast(`"${name}" ${t('deleted')}`);
  refreshSkillList();
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
        <div style="flex:1">
          <div class="card-title">${t('checkUpdate')}</div>
          <div class="card-meta" id="update-status">--</div>
        </div>
        <button class="btn btn-primary btn-sm" onclick="doCheckUpdate()">${t('checkUpdate')}</button>
      </div>
    </div>

    <div class="card">
      <div class="card-row">
        <div style="flex:1">
          <div class="card-title">${t('language')}</div>
        </div>
        <select id="lang-select" style="padding:6px 10px;background:var(--bg);border:1px solid var(--border);border-radius:6px;color:var(--text);font-size:13px" onchange="switchLang(this.value)">
          <option value="zh-CN">${t('langZh')}</option>
          <option value="en-US">${t('langEn')}</option>
        </select>
      </div>
    </div>
  `;
  document.getElementById('lang-select').value = lang;
  showVersion();
}

async function showVersion() {
  try {
    const v = await invoke('get_version');
    const el = document.getElementById('version-label');
    if (el) el.textContent = 'v' + v;
  } catch (_) {}
}

async function doCheckUpdate() {
  const status = document.getElementById('update-status');
  if (status) status.textContent = t('checking');
  try {
    const msg = await invoke('check_update');
    if (status) status.textContent = msg;
    toast(msg);
  } catch (e) {
    if (status) status.textContent = t('updateFailed');
    toast(t('updateFailed') + ': ' + e, 'error');
  }
}

async function switchLang(value) {
  lang = value;
  try { await invoke('save_language', { lang: value }); } catch (_) {}
  // Reload current page to apply translations
  const active = document.querySelector('.nav-btn.active');
  if (active) loadPage(active.dataset.page);
}

// ═══════════════════════════════════════════
// Init
// ═══════════════════════════════════════════

(async () => {
  await initLang();
  document.getElementById('lang-select')?.value = lang;
  loadPage('agents');
  showVersion();
})();
