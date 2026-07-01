// ── Navigation ──

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
    case 'launch': renderLaunch(); break;
  }
}

// ── Toast ──

function toast(msg, type = 'success') {
  const el = document.createElement('div');
  el.className = `toast toast-${type}`;
  el.textContent = msg;
  document.body.appendChild(el);
  setTimeout(() => el.remove(), 2500);
}

// ── Helpers ──

function maskKey(key) {
  if (!key || key.length <= 8) return '****';
  return key.slice(0, 4) + '...' + key.slice(-4);
}

// ═══════════════════════════════════════════
// AGENTS PAGE
// ═══════════════════════════════════════════

async function renderAgents() {
  const content = document.getElementById('content');
  content.innerHTML = `
    <div class="page-title">📡 Agents</div>
    <button class="btn btn-primary" onclick="scanAndShow()">🔍 扫描已安装的 Agent</button>
    <div id="agent-list" style="margin-top:20px">
      <div class="empty">点击上方按钮扫描</div>
    </div>
  `;
}

async function scanAndShow() {
  const list = document.getElementById('agent-list');
  list.innerHTML = '<div class="empty">扫描中...</div>';

  try {
    const agents = await window.__TAURI__.invoke('scan_agents');
    if (agents.length === 0) {
      list.innerHTML = '<div class="empty">未检测到任何 Agent</div>';
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
  } catch (e) {
    list.innerHTML = '<div class="empty">扫描出错: ' + e + '</div>';
  }
}

// ═══════════════════════════════════════════
// PROFILES PAGE
// ═══════════════════════════════════════════

async function renderProfiles() {
  const content = document.getElementById('content');
  content.innerHTML = `
    <div class="page-title">⚙️ Profiles</div>
    <button class="btn btn-primary" onclick="openProfileModal()">+ 新建 Profile</button>
    <div id="profile-list" style="margin-top:20px"></div>
  `;
  refreshProfileList();
}

async function refreshProfileList() {
  const list = document.getElementById('profile-list');
  try {
    const [profiles, defaultName] = await Promise.all([
      window.__TAURI__.invoke('list_profiles'),
      window.__TAURI__.invoke('get_default_profile'),
    ]);
    if (profiles.length === 0) {
      list.innerHTML = '<div class="empty">还没有 Profile，点击上方按钮创建</div>';
      return;
    }
    list.innerHTML = profiles.map(p => `
      <div class="card">
        <div class="card-row">
          <div style="flex:1">
            <div class="card-title">${p.name} ${p.name === defaultName ? '⭐' : ''}</div>
            <div class="card-meta">${p.format} | ${p.model} | ${p.base_url}</div>
          </div>
          <button class="btn-sm" onclick="openProfileModal('${p.name}')">编辑</button>
          <button class="btn-sm" style="color:var(--danger)" onclick="delProfile('${p.name}')">删除</button>
          ${p.name !== defaultName ? `<button class="btn-sm" onclick="setDefaultProfile('${p.name}')">设默认</button>` : ''}
        </div>
      </div>
    `).join('');
  } catch (e) {
    list.innerHTML = '<div class="empty">加载出错: ' + e + '</div>';
  }
}

function openProfileModal(name = null) {
  const isEdit = !!name;
  const modal = document.createElement('div');
  modal.className = 'modal-overlay';
  modal.innerHTML = `
    <div class="modal">
      <h3>${isEdit ? '编辑 Profile' : '新建 Profile'}</h3>
      ${isEdit ? '' : '<div class="form-group"><label>名称</label><input id="pf-name" placeholder="my-profile"></div>'}
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
        <button class="btn btn-primary" id="pf-save">保存</button>
        <button class="btn" onclick="this.closest('.modal-overlay').remove()">取消</button>
      </div>
    </div>
  `;
  document.body.appendChild(modal);

  // Fill existing data if editing
  if (isEdit) {
    window.__TAURI__.invoke('get_profile', { name }).then(p => {
      if (p) {
        document.getElementById('pf-format').value = p.format;
        document.getElementById('pf-url').value = p.base_url;
        document.getElementById('pf-model').value = p.model;
        document.getElementById('pf-key').value = p.api_key;
        if (p.max_tokens) document.getElementById('pf-tokens').value = p.max_tokens;
      }
    });
  }

  modal.querySelector('#pf-save').addEventListener('click', async () => {
    const profile = {
      name: isEdit ? name : document.getElementById('pf-name').value,
      format: document.getElementById('pf-format').value,
      base_url: document.getElementById('pf-url').value,
      model: document.getElementById('pf-model').value,
      api_key: document.getElementById('pf-key').value,
      max_tokens: parseInt(document.getElementById('pf-tokens').value) || null,
      extra: {},
    };
    try {
      await window.__TAURI__.invoke('save_profile', { profile });
      modal.remove();
      toast(isEdit ? `Profile '${profile.name}' 已更新` : `Profile '${profile.name}' 已创建`);
      refreshProfileList();
    } catch (e) {
      toast('保存失败: ' + e, 'error');
    }
  });

  modal.addEventListener('click', (e) => { if (e.target === modal) modal.remove(); });
}

async function delProfile(name) {
  if (!confirm(`确认删除 Profile "${name}"？`)) return;
  try {
    await window.__TAURI__.invoke('delete_profile', { name });
    toast(`Profile "${name}" 已删除`);
    refreshProfileList();
  } catch (e) {
    toast('删除失败: ' + e, 'error');
  }
}

async function setDefaultProfile(name) {
  try {
    await window.__TAURI__.invoke('set_default_profile', { name });
    toast(`默认 Profile = "${name}"`);
    refreshProfileList();
  } catch (e) {
    toast('设置失败: ' + e, 'error');
  }
}

// ═══════════════════════════════════════════
// SKILLS PAGE
// ═══════════════════════════════════════════

async function renderSkills() {
  const content = document.getElementById('content');
  content.innerHTML = `
    <div class="page-title">📚 Skills</div>
    <button class="btn btn-primary" onclick="openSkillModal()">+ 新建 Skill</button>
    <div id="skill-list" style="margin-top:20px"></div>
  `;
  refreshSkillList();
}

async function refreshSkillList() {
  const list = document.getElementById('skill-list');
  try {
    const skills = await window.__TAURI__.invoke('list_skills');
    if (skills.length === 0) {
      list.innerHTML = '<div class="empty">还没有 Skill，点击上方按钮创建</div>';
      return;
    }
    list.innerHTML = skills.map(s => `
      <div class="card">
        <div>
          <div class="card-title">${s.name} <span style="font-weight:400;color:var(--text-dim)">v${s.version}</span></div>
          <div class="card-meta">${s.description}</div>
          <div style="margin-top:8px;font-size:12px;color:var(--text-dim);max-height:40px;overflow:hidden">${s.prompt.slice(0, 120)}</div>
        </div>
        <div style="margin-top:10px;display:flex;gap:8px">
          <button class="btn-sm" onclick="openSkillModal('${s.name}')">编辑</button>
          <button class="btn-sm" style="color:var(--danger)" onclick="delSkill('${s.name}')">删除</button>
        </div>
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
      <div class="form-group"><label>描述</label><input id="sk-desc" placeholder="一句话描述"></div>
      <div class="form-group"><label>版本</label><input id="sk-ver" value="1.0"></div>
      <div class="form-group"><label>Prompt</label><textarea id="sk-prompt" placeholder="输入 Skill 的完整 prompt..."></textarea></div>
      <div class="form-actions">
        <button class="btn btn-primary" id="sk-save">保存</button>
        <button class="btn" onclick="this.closest('.modal-overlay').remove()">取消</button>
      </div>
    </div>
  `;
  document.body.appendChild(modal);

  if (isEdit) {
    window.__TAURI__.invoke('get_skill', { name }).then(s => {
      if (s) {
        document.getElementById('sk-desc').value = s.description;
        document.getElementById('sk-ver').value = s.version;
        document.getElementById('sk-prompt').value = s.prompt;
      }
    });
  }

  modal.querySelector('#sk-save').addEventListener('click', async () => {
    const skill = {
      name: isEdit ? name : document.getElementById('sk-name').value,
      description: document.getElementById('sk-desc').value,
      version: document.getElementById('sk-ver').value,
      prompt: document.getElementById('sk-prompt').value,
    };
    try {
      await window.__TAURI__.invoke('save_skill', { skill });
      modal.remove();
      toast(isEdit ? `Skill '${skill.name}' 已更新` : `Skill '${skill.name}' 已创建`);
      refreshSkillList();
    } catch (e) {
      toast('保存失败: ' + e, 'error');
    }
  });

  modal.addEventListener('click', (e) => { if (e.target === modal) modal.remove(); });
}

async function delSkill(name) {
  if (!confirm(`确认删除 Skill "${name}"？`)) return;
  try {
    await window.__TAURI__.invoke('delete_skill', { name });
    toast(`Skill "${name}" 已删除`);
    refreshSkillList();
  } catch (e) {
    toast('删除失败: ' + e, 'error');
  }
}

// ═══════════════════════════════════════════
// LAUNCH PAGE
// ═══════════════════════════════════════════

async function renderLaunch() {
  const content = document.getElementById('content');
  try {
    const [agents, profiles] = await Promise.all([
      window.__TAURI__.invoke('scan_agents'),
      window.__TAURI__.invoke('list_profiles'),
    ]);
    if (agents.length === 0) {
      content.innerHTML = '<div class="page-title">🚀 Launch</div><div class="empty">未检测到 Agent，请先在 Agents 页面扫描</div>';
      return;
    }
    if (profiles.length === 0) {
      content.innerHTML = '<div class="page-title">🚀 Launch</div><div class="empty">还没有 Profile，请先在 Profiles 页面创建</div>';
      return;
    }
    content.innerHTML = `
      <div class="page-title">🚀 Launch</div>
      <div class="card launch-card">
        <div class="form-group">
          <label>选择 Agent</label>
          <select id="launch-agent">
            ${agents.map(a => `<option value="${a.binary}">${a.name} (${a.binary})</option>`).join('')}
          </select>
        </div>
        <div class="form-group">
          <label>选择 Profile</label>
          <select id="launch-profile">
            ${profiles.map(p => `<option value="${p.name}">${p.name} — ${p.format} | ${p.model}</option>`).join('')}
          </select>
        </div>
        <button class="btn btn-primary" onclick="doLaunch()" style="margin-top:8px">🚀 启动</button>
        <div id="launch-result" style="margin-top:12px;font-size:13px"></div>
      </div>
    `;
  } catch (e) {
    content.innerHTML = '<div class="page-title">🚀 Launch</div><div class="empty">加载出错: ' + e + '</div>';
  }
}

async function doLaunch() {
  const agent = document.getElementById('launch-agent').value;
  const profile = document.getElementById('launch-profile').value;
  const result = document.getElementById('launch-result');
  result.innerHTML = '启动中...';
  try {
    const code = await window.__TAURI__.invoke('launch_agent', { agent, profile });
    result.innerHTML = code === 0
      ? '<span style="color:var(--success)">Agent 已退出</span>'
      : `<span style="color:var(--danger)">Agent 退出码: ${code}</span>`;
  } catch (e) {
    result.innerHTML = `<span style="color:var(--danger)">启动失败: ${e}</span>`;
  }
}

// ── Init ──
loadPage('agents');
showVersion();

// ═══════════════════════════════════════════
// UPDATE CHECK
// ═══════════════════════════════════════════

async function showVersion() {
  try {
    const v = await window.__TAURI__.invoke('get_version');
    document.getElementById('version-label').textContent = 'v' + v;
  } catch (_) {}
}

async function checkUpdate() {
  const label = document.getElementById('version-label');
  label.textContent = '检查中...';
  try {
    const msg = await window.__TAURI__.invoke('check_update');
    toast(msg);
    showVersion();
  } catch (e) {
    toast('检查更新失败: ' + e, 'error');
    showVersion();
  }
}
