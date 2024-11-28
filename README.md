# Mouse Mover Automático para Prevenir Bloqueio de Tela

Este projeto foi criado para evitar que a tela do computador bloqueie automaticamente devido à inatividade. Ele movimenta o cursor do mouse horizontalmente de forma contínua, com o tempo de execução configurável pelo usuário. Além disso, o programa pode ser interrompido manualmente ao pressionar a tecla **`B`**.

---

## Como Funciona

1. **Movimento Automático do Mouse**:
   - O cursor do mouse se move horizontalmente entre duas posições predefinidas (`x = 500` e `x = 1500`).
   - A direção do movimento é invertida automaticamente ao atingir os limites.

2. **Configuração de Tempo**:
   - No início, o programa solicita o tempo (em segundos) durante o qual o mouse deve se mover automaticamente.
   - O programa para automaticamente ao atingir o tempo definido.

3. **Interrupção Manual**:
   - Durante a execução, você pode pressionar a tecla **`B`** para parar o programa imediatamente.

---

## Como Usar

### Pré-requisitos
- **Rust** instalado no sistema. Para instalar, siga as instruções em [https://rustup.rs/](https://rustup.rs/).

### Passos para Executar
1. Clone o repositório:
   ```bash
   git clone <URL_DO_REPOSITORIO>
   cd <NOME_DO_DIRETORIO>
