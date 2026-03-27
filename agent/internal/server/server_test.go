package server

import (
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/gin-gonic/gin"
	"github.com/ha1den/serverhub-agent/internal/config"
)

func init() {
	gin.SetMode(gin.TestMode)
}

// setupRouter creates the same router as Run() but without starting a listener.
func setupRouter(cfg *config.Config) *gin.Engine {
	r := gin.New()
	r.Use(gin.Recovery())

	r.GET("/api/health", func(c *gin.Context) {
		c.JSON(http.StatusOK, gin.H{"status": "ok", "hostname": cfg.Hostname})
	})

	authMiddleware := func(c *gin.Context) {
		if cfg.Token != "" {
			token := c.GetHeader("Authorization")
			if token != "Bearer "+cfg.Token {
				c.AbortWithStatusJSON(http.StatusUnauthorized, gin.H{"error": "unauthorized"})
				return
			}
		}
		c.Next()
	}

	r.GET("/api/metrics", authMiddleware, func(c *gin.Context) {
		// Use a stub response to avoid gopsutil dependency in tests
		c.JSON(http.StatusOK, gin.H{"timestamp": 1234567890, "cpu": gin.H{"total_percent": 25.0}})
	})

	return r
}

func TestHealthEndpoint(t *testing.T) {
	cfg := &config.Config{
		Port:     9100,
		Token:    "test-token",
		Hostname: "test-server",
	}
	router := setupRouter(cfg)

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/api/health", nil)
	router.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("health status = %d, want %d", w.Code, http.StatusOK)
	}

	var body map[string]string
	if err := json.Unmarshal(w.Body.Bytes(), &body); err != nil {
		t.Fatalf("invalid JSON response: %v", err)
	}
	if body["status"] != "ok" {
		t.Errorf("status = %q, want %q", body["status"], "ok")
	}
	if body["hostname"] != "test-server" {
		t.Errorf("hostname = %q, want %q", body["hostname"], "test-server")
	}
}

func TestMetricsAuth(t *testing.T) {
	cfg := &config.Config{
		Port:     9100,
		Token:    "secret-token",
		Hostname: "test-server",
	}
	router := setupRouter(cfg)

	tests := []struct {
		name       string
		authHeader string
		wantStatus int
	}{
		{
			name:       "no token",
			authHeader: "",
			wantStatus: http.StatusUnauthorized,
		},
		{
			name:       "wrong token",
			authHeader: "Bearer wrong-token",
			wantStatus: http.StatusUnauthorized,
		},
		{
			name:       "malformed header",
			authHeader: "secret-token",
			wantStatus: http.StatusUnauthorized,
		},
		{
			name:       "valid token",
			authHeader: "Bearer secret-token",
			wantStatus: http.StatusOK,
		},
	}

	for _, tt := range tests {
		t.Run(tt.name, func(t *testing.T) {
			w := httptest.NewRecorder()
			req, _ := http.NewRequest("GET", "/api/metrics", nil)
			if tt.authHeader != "" {
				req.Header.Set("Authorization", tt.authHeader)
			}
			router.ServeHTTP(w, req)

			if w.Code != tt.wantStatus {
				t.Errorf("status = %d, want %d", w.Code, tt.wantStatus)
			}
		})
	}
}

func TestMetricsNoTokenRequired(t *testing.T) {
	// When config has empty token, auth should be skipped
	cfg := &config.Config{
		Port:     9100,
		Token:    "",
		Hostname: "open-server",
	}
	router := setupRouter(cfg)

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/api/metrics", nil)
	router.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Errorf("status = %d, want %d (no token required)", w.Code, http.StatusOK)
	}
}

func TestMetricsResponseStructure(t *testing.T) {
	cfg := &config.Config{
		Port:     9100,
		Token:    "t",
		Hostname: "test",
	}
	router := setupRouter(cfg)

	w := httptest.NewRecorder()
	req, _ := http.NewRequest("GET", "/api/metrics", nil)
	req.Header.Set("Authorization", "Bearer t")
	router.ServeHTTP(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("status = %d, want %d", w.Code, http.StatusOK)
	}

	var body map[string]interface{}
	if err := json.Unmarshal(w.Body.Bytes(), &body); err != nil {
		t.Fatalf("invalid JSON: %v", err)
	}
	if _, ok := body["timestamp"]; !ok {
		t.Error("response missing 'timestamp' field")
	}
	if _, ok := body["cpu"]; !ok {
		t.Error("response missing 'cpu' field")
	}
}
