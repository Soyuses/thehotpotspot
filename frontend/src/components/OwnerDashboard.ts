// Owner Dashboard component
import { apiClient } from '@/services/api';
import { formatCurrency, formatTokens, formatDate, formatPercentage } from '@/utils/format';
import type { UTHolder, ConversionResult, TokenomicsConfig } from '@/types/api';

export class OwnerDashboard {
  private container: HTMLElement;
  private currentTab: string = 'monitoring';
  private utHolders: UTHolder[] = [];
  private selectedHolders: Set<string> = new Set();

  constructor(container: HTMLElement) {
    this.container = container;
  }

  async render(): Promise<void> {
    this.container.innerHTML = `
      <div class="owner-dashboard">
        <header class="dashboard-header">
          <h1>👑 Панель владельца сети</h1>
          <div class="header-actions">
            <button class="btn btn-secondary" id="refreshBtn">🔄 Обновить</button>
            <button class="btn btn-primary" id="settingsBtn">⚙️ Настройки</button>
          </div>
        </header>

        <nav class="tabs">
          <button class="tab active" data-tab="monitoring">Мониторинг</button>
          <button class="tab" data-tab="franchises">Франшизы</button>
          <button class="tab" data-tab="token-emission">Эмиссия токенов</button>
          <button class="tab" data-tab="cost-accounting">Учет затрат</button>
          <button class="tab" data-tab="streams">Управление стримами</button>
          <button class="tab" data-tab="image-management">Управление изображениями</button>
          <button class="tab" data-tab="unclaimed">Невостребованные</button>
          <button class="tab" data-tab="security">Безопасность</button>
        </nav>

        <main class="tab-content">
          <div id="monitoring" class="tab-panel active">
            ${this.renderMonitoringTab()}
          </div>
          <div id="franchises" class="tab-panel">
            ${this.renderFranchisesTab()}
          </div>
          <div id="token-emission" class="tab-panel">
            ${this.renderTokenEmissionTab()}
          </div>
          <div id="cost-accounting" class="tab-panel">
            ${this.renderCostAccountingTab()}
          </div>
          <div id="streams" class="tab-panel">
            ${this.renderStreamsTab()}
          </div>
          <div id="image-management" class="tab-panel">
            ${this.renderImageManagementTab()}
          </div>
          <div id="unclaimed" class="tab-panel">
            ${this.renderUnclaimedTab()}
          </div>
          <div id="security" class="tab-panel">
            ${this.renderSecurityTab()}
          </div>
        </main>
      </div>
    `;

    await this.loadData();
    this.attachEventListeners();
  }

  private renderMonitoringTab(): string {
    return `
      <div class="monitoring-grid">
        <div class="card">
          <h3>Распределение токенов</h3>
          <div class="token-distribution">
            <div class="progress-bar">
              <div class="progress-fill" id="ownerProgress" style="width: 48%"></div>
            </div>
            <p>Владелец: <span id="ownerTokensPercent">48.00%</span></p>
            
            <div class="progress-bar">
              <div class="progress-fill" id="charityProgress" style="width: 3%; background: #28a745;"></div>
            </div>
            <p>Благотворительный фонд: <span id="charityTokensPercent">3.00%</span></p>
            
            <div class="progress-bar">
              <div class="progress-fill" id="othersProgress" style="width: 49%; background: #17a2b8;"></div>
            </div>
            <p>Остальные: <span id="othersTokensPercent">49.00%</span></p>
          </div>
        </div>

        <div class="card">
          <h3>Токеномика системы</h3>
          <div class="token-ratio">
            <div class="ratio-item">
              <h4>Security Tokens (ST)</h4>
              <div class="ratio-value" id="securityTokensCount">0</div>
              <div class="ratio-percentage">Генерируются за продажи</div>
            </div>
            <div class="ratio-separator">≠</div>
            <div class="ratio-item">
              <h4>Utility Tokens (SPOT)</h4>
              <div class="ratio-value" id="utilityTokensCount">0</div>
              <div class="ratio-percentage">Генерируются за активность</div>
            </div>
          </div>
        </div>

        <div class="card">
          <h3>Статистика сети</h3>
          <div class="stats-grid">
            <div class="stat">
              <span class="stat-value" id="activeAlerts">3</span>
              <span class="stat-label">Активных алертов</span>
            </div>
            <div class="stat">
              <span class="stat-value" id="franchiseNodes">6</span>
              <span class="stat-label">Франшизных узлов</span>
            </div>
            <div class="stat">
              <span class="stat-value" id="charityFund">150.00</span>
              <span class="stat-label">Благотворительный фонд (GEL)</span>
            </div>
            <div class="stat">
              <span class="stat-value" id="unclaimedTokens">245.00</span>
              <span class="stat-label">Невостребованные токены</span>
            </div>
          </div>
        </div>
      </div>
    `;
  }

  private renderTokenEmissionTab(): string {
    return `
      <div class="token-emission">
        <div class="card">
          <h3>🔄 Конвертация UT (SPOT) в ST (THP)</h3>
          <div class="conversion-section">
            <div class="form-group">
              <label>Выберите держателей UT (SPOT):</label>
              <div class="ut-holders-list">
                <div class="ut-holder-item">
                  <input type="checkbox" id="selectAll" />
                  <label for="selectAll"><strong>Выбрать всех</strong></label>
                </div>
                <div id="utHoldersList">
                  <p>Загрузка списка держателей UT...</p>
                </div>
              </div>
            </div>
            
            <div class="form-group">
              <label>Курс обмена (SPOT → ST):</label>
              <div class="exchange-rate">
                <input type="number" id="exchangeRate" value="10" min="1" max="1000" step="1" />
                <span>SPOT = 1 ST</span>
              </div>
              <small>Укажите, сколько SPOT токенов нужно для получения 1 ST токена</small>
            </div>
            
            <div class="conversion-summary">
              <h4>Сводка конвертации:</h4>
              <div id="conversionSummary">
                <p>Выберите держателей UT для просмотра сводки</p>
              </div>
            </div>
            
            <div class="form-group">
              <button type="button" class="btn btn-primary" id="convertBtn" disabled>
                🔄 Выполнить конвертацию
              </button>
              <button type="button" class="btn btn-secondary" id="clearSelectionBtn">
                🗑️ Очистить выбор
              </button>
            </div>
          </div>
        </div>
      </div>
    `;
  }

  private renderFranchisesTab(): string {
    return `
      <div class="franchises">
        <div class="card">
          <h3>Франшизные узлы</h3>
          <div id="franchisesList">
            <p>Загрузка списка франшиз...</p>
          </div>
        </div>
      </div>
    `;
  }

  private renderCostAccountingTab(): string {
    return `
      <div class="cost-accounting">
        <div class="card">
          <h3>Система учета затрат</h3>
          <div class="cost-summary">
            <div class="cost-item">
              <span class="cost-label">Сырье и ингредиенты:</span>
              <span class="cost-value" id="ingredientsCost">0.00 GEL</span>
            </div>
            <div class="cost-item">
              <span class="cost-label">Аренда и коммунальные:</span>
              <span class="cost-value" id="rentCost">0.00 GEL</span>
            </div>
            <div class="cost-item">
              <span class="cost-label">Зарплата персонала:</span>
              <span class="cost-value" id="salaryCost">0.00 GEL</span>
            </div>
            <div class="cost-item">
              <span class="cost-label">Маркетинг и реклама:</span>
              <span class="cost-value" id="marketingCost">0.00 GEL</span>
            </div>
            <div class="cost-item total">
              <span class="cost-label">Общие затраты:</span>
              <span class="cost-value" id="totalCost">0.00 GEL</span>
            </div>
          </div>
        </div>
      </div>
    `;
  }

  private renderStreamsTab(): string {
    return `
      <div class="streams">
        <div class="card">
          <h3>Управление стримами</h3>
          <div id="streamsList">
            <p>Загрузка списка стримов...</p>
          </div>
        </div>
      </div>
    `;
  }

  private renderImageManagementTab(): string {
    return `
      <div class="image-management">
        <div class="card">
          <h3>Управление изображениями</h3>
          <div class="upload-section">
            <input type="file" id="imageUpload" accept="image/*" multiple />
            <button class="btn btn-primary" id="uploadBtn">Загрузить изображения</button>
          </div>
          <div id="uploadedImages">
            <p>Загруженные изображения появятся здесь</p>
          </div>
        </div>
      </div>
    `;
  }

  private renderUnclaimedTab(): string {
    return `
      <div class="unclaimed">
        <div class="card">
          <h3>Невостребованные токены</h3>
          <div id="unclaimedList">
            <p>Загрузка списка невостребованных токенов...</p>
          </div>
        </div>
      </div>
    `;
  }

  private renderSecurityTab(): string {
    return `
      <div class="security">
        <div class="card">
          <h3>Безопасность</h3>
          <div id="securityReport">
            <p>Загрузка отчета по безопасности...</p>
          </div>
        </div>
      </div>
    `;
  }

  private async loadData(): Promise<void> {
    try {
      // Load UT holders for conversion
      this.utHolders = await apiClient.getUTHolders();
      this.renderUTHolders();

      // Load other data based on current tab
      await this.loadTabData(this.currentTab);
    } catch (error) {
      console.error('Error loading owner dashboard data:', error);
      this.showError('Ошибка загрузки данных');
    }
  }

  private async loadTabData(tab: string): Promise<void> {
    switch (tab) {
      case 'monitoring':
        await this.loadMonitoringData();
        break;
      case 'franchises':
        await this.loadFranchisesData();
        break;
      case 'streams':
        await this.loadStreamsData();
        break;
      // Add other tabs as needed
    }
  }

  private async loadMonitoringData(): Promise<void> {
    // Load monitoring data
    // This would typically fetch real data from the API
  }

  private async loadFranchisesData(): Promise<void> {
    // Load franchises data
  }

  private async loadStreamsData(): Promise<void> {
    // Load streams data
  }

  private renderUTHolders(): void {
    const container = document.getElementById('utHoldersList');
    if (!container) return;

    if (this.utHolders.length === 0) {
      container.innerHTML = '<p>Нет держателей UT</p>';
      return;
    }

    container.innerHTML = this.utHolders.map((holder, index) => `
      <div class="ut-holder-item">
        <input type="checkbox" id="holder_${index}" value="${holder.address}" />
        <label for="holder_${index}">
          <span>${holder.name || holder.address}</span>
          <span class="ut-balance">${formatTokens(holder.balance, 'UT')}</span>
        </label>
      </div>
    `).join('');
  }

  private attachEventListeners(): void {
    // Tab switching
    document.querySelectorAll('.tab').forEach(tab => {
      tab.addEventListener('click', (e) => {
        const target = e.target as HTMLElement;
        const tabName = target.dataset.tab;
        if (tabName) {
          this.switchTab(tabName);
        }
      });
    });

    // UT holders selection
    document.getElementById('selectAll')?.addEventListener('change', (e) => {
      const target = e.target as HTMLInputElement;
      const checkboxes = document.querySelectorAll('#utHoldersList input[type="checkbox"]') as NodeListOf<HTMLInputElement>;
      
      checkboxes.forEach(checkbox => {
        checkbox.checked = target.checked;
        if (target.checked) {
          this.selectedHolders.add(checkbox.value);
        } else {
          this.selectedHolders.delete(checkbox.value);
        }
      });
      
      this.updateConversionSummary();
    });

    // Individual holder selection
    document.getElementById('utHoldersList')?.addEventListener('change', (e) => {
      const target = e.target as HTMLInputElement;
      if (target.type === 'checkbox') {
        if (target.checked) {
          this.selectedHolders.add(target.value);
        } else {
          this.selectedHolders.delete(target.value);
        }
        this.updateConversionSummary();
      }
    });

    // Exchange rate change
    document.getElementById('exchangeRate')?.addEventListener('input', () => {
      this.updateConversionSummary();
    });

    // Conversion button
    document.getElementById('convertBtn')?.addEventListener('click', () => {
      this.executeConversion();
    });

    // Clear selection button
    document.getElementById('clearSelectionBtn')?.addEventListener('click', () => {
      this.clearSelection();
    });

    // Refresh button
    document.getElementById('refreshBtn')?.addEventListener('click', () => {
      this.loadData();
    });
  }

  private switchTab(tabName: string): void {
    // Update tab buttons
    document.querySelectorAll('.tab').forEach(tab => {
      tab.classList.remove('active');
    });
    document.querySelector(`[data-tab="${tabName}"]`)?.classList.add('active');

    // Update tab panels
    document.querySelectorAll('.tab-panel').forEach(panel => {
      panel.classList.remove('active');
    });
    document.getElementById(tabName)?.classList.add('active');

    this.currentTab = tabName;
    this.loadTabData(tabName);
  }

  private updateConversionSummary(): void {
    const summaryContainer = document.getElementById('conversionSummary');
    const exchangeRateInput = document.getElementById('exchangeRate') as HTMLInputElement;
    const convertBtn = document.getElementById('convertBtn') as HTMLButtonElement;

    if (!summaryContainer || !exchangeRateInput || !convertBtn) return;

    const exchangeRate = parseInt(exchangeRateInput.value) || 10;

    if (this.selectedHolders.size === 0) {
      summaryContainer.innerHTML = '<p>Выберите держателей UT для просмотра сводки</p>';
      convertBtn.disabled = true;
      return;
    }

    let totalSPOT = 0;
    let totalST = 0;
    const conversionDetails: Array<{ name: string; spot: number; st: number }> = [];

    this.selectedHolders.forEach(address => {
      const holder = this.utHolders.find(h => h.address === address);
      if (holder) {
        const stToReceive = Math.floor(holder.balance / exchangeRate);
        totalSPOT += holder.balance;
        totalST += stToReceive;
        conversionDetails.push({
          name: holder.name || holder.address,
          spot: holder.balance,
          st: stToReceive
        });
      }
    });

    let summaryHTML = `
      <div class="conversion-item">
        <span>Всего выбрано держателей:</span>
        <span>${this.selectedHolders.size}</span>
      </div>
    `;

    conversionDetails.forEach(detail => {
      summaryHTML += `
        <div class="conversion-item">
          <span>${detail.name}:</span>
          <span>${formatTokens(detail.spot, 'UT')} → ${formatTokens(detail.st, 'ST')}</span>
        </div>
      `;
    });

    summaryHTML += `
      <div class="conversion-item">
        <span>Общая сумма:</span>
        <span>${formatTokens(totalSPOT, 'UT')} → ${formatTokens(totalST, 'ST')}</span>
      </div>
    `;

    summaryContainer.innerHTML = summaryHTML;
    convertBtn.disabled = false;
  }

  private async executeConversion(): Promise<void> {
    if (this.selectedHolders.size === 0) {
      this.showError('Выберите хотя бы одного держателя UT');
      return;
    }

    const exchangeRateInput = document.getElementById('exchangeRate') as HTMLInputElement;
    const exchangeRate = parseInt(exchangeRateInput.value) || 10;

    const confirmMessage = `Вы уверены, что хотите конвертировать UT в ST по курсу ${exchangeRate} SPOT = 1 ST для ${this.selectedHolders.size} держателей?`;

    if (!confirm(confirmMessage)) {
      return;
    }

    try {
      const result = await apiClient.convertUTToST({
        holders: Array.from(this.selectedHolders),
        exchange_rate: exchangeRate
      });

      if (result?.success) {
        this.showSuccess('Конвертация выполнена успешно!');
        this.clearSelection();
        await this.loadData(); // Reload data
      } else {
        this.showError('Ошибка при выполнении конвертации');
      }
    } catch (error) {
      console.error('Conversion error:', error);
      this.showError('Ошибка при выполнении конвертации');
    }
  }

  private clearSelection(): void {
    this.selectedHolders.clear();
    document.getElementById('selectAll')?.setAttribute('checked', 'false');
    const checkboxes = document.querySelectorAll('#utHoldersList input[type="checkbox"]') as NodeListOf<HTMLInputElement>;
    checkboxes.forEach(checkbox => {
      checkbox.checked = false;
    });
    this.updateConversionSummary();
  }

  private showError(message: string): void {
    // Implement error notification
    console.error(message);
    alert(message); // Replace with proper notification component
  }

  private showSuccess(message: string): void {
    // Implement success notification
    console.log(message);
    alert(message); // Replace with proper notification component
  }
}
