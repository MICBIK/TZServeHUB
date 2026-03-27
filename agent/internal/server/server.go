package server

import (
	"fmt"
	"net/http"

	"github.com/gin-gonic/gin"
	"github.com/ha1den/serverhub-agent/internal/collector"
	"github.com/ha1den/serverhub-agent/internal/config"
)

func Run(cfg *config.Config) error {
	gin.SetMode(gin.ReleaseMode)
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
		metrics, err := collector.Collect()
		if err != nil {
			c.JSON(http.StatusInternalServerError, gin.H{"error": err.Error()})
			return
		}
		c.JSON(http.StatusOK, metrics)
	})

	return r.Run(fmt.Sprintf(":%d", cfg.Port))
}
