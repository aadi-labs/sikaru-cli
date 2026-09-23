"""Harbor custom agent import path: sikaru_eval.harbor:SikaruAgent."""

from harbor.agents.base import BaseAgent

from .agent import SikaruAgentMixin


class SikaruAgent(SikaruAgentMixin, BaseAgent):
    pass
