"""Pier custom agent import path: sikaru_eval.pier:SikaruAgent."""

from urllib.parse import urlsplit

from pier.agents.base import BaseAgent
from pier.models.agent.network import NetworkAllowlist

from .agent import SikaruAgentMixin


class SikaruAgent(SikaruAgentMixin, BaseAgent):
    def network_allowlist(self):
        return NetworkAllowlist(domains=[urlsplit(self.base_url).hostname])
