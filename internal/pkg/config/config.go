package config

import (
	"errors"
	"fmt"

	"github.com/pelletier/go-toml"
)

const filename = "ocean.toml"

type Config struct {
	Database database
}

type database struct {
	Name     string
	Host     string
	Port     int
	User     string
	Password string
}

func NewConfig() (*Config, error) {
	c, err := toml.LoadFile(filename)

	if err != nil {
		return nil, errors.New("failed to read config file: " + err.Error())
	}

	config := &Config{}
	c.Unmarshal(config)

	fmt.Println(config)

	return config, nil
}
